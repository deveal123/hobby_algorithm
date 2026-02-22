use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item};

pub struct ModuleIndex {
    // module_path (e.g., "algorithm::io::reader") -> file_path
    pub modules: HashMap<String, PathBuf>,
}

impl ModuleIndex {
    pub fn new(root_dir: &Path) -> Self {
        let mut modules = HashMap::new();

        // Assume src/lib.rs is the entry point for "algorithm"
        let lib_path = root_dir.join("src/lib.rs");
        if lib_path.exists() {
            modules.insert("algorithm".to_string(), lib_path.clone());
            Self::scan_module(&lib_path, "algorithm", &mut modules, root_dir);
        }

        ModuleIndex { modules }
    }

    fn scan_module(
        file_path: &Path,
        current_mod_path: &str,
        modules: &mut HashMap<String, PathBuf>,
        root_dir: &Path,
    ) {
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return,
        };

        let ast = match syn::parse_file(&content) {
            Ok(ast) => ast,
            Err(_) => return,
        };

        for item in ast.items {
            if let Item::Mod(item_mod) = item {
                let mod_name = item_mod.ident.to_string();
                let full_mod_path = format!("{}::{}", current_mod_path, mod_name);

                // Inline modules are already in the file, we skip them for file mapping unless we want to support extracting them?
                // For now, only look for external modules `mod foo;`
                if item_mod.content.is_none() {
                    // Look for file
                    // 1. dir/mod_name.rs
                    // 2. dir/mod_name/mod.rs
                    let parent_dir = file_path.parent().unwrap();
                    let path1 = parent_dir.join(format!("{}.rs", mod_name));
                    let path2 = parent_dir.join(&mod_name).join("mod.rs");

                    if path1.exists() {
                        modules.insert(full_mod_path.clone(), path1.clone());
                        Self::scan_module(&path1, &full_mod_path, modules, root_dir);
                    } else if path2.exists() {
                        modules.insert(full_mod_path.clone(), path2.clone());
                        Self::scan_module(&path2, &full_mod_path, modules, root_dir);
                    }
                }
            }
        }
    }
}

pub struct Resolver {
    index: ModuleIndex,
}

impl Resolver {
    pub fn new(index: ModuleIndex) -> Self {
        Self { index }
    }

    pub fn resolve(&self, entry_file: &Path) -> HashSet<String> {
        let mut required_modules = HashSet::new();
        let content = fs::read_to_string(entry_file).expect("Cannot read entry file");
        let ast = syn::parse_file(&content).expect("Cannot parse entry file");

        // Entry file is external, so current_mod is empty or we assume "algorithm" context if it was inside?
        // But entry file USES algorithm. So context is nothing.
        self.visit_ast(&ast, &mut required_modules, "");

        // Transitively resolve dependencies for each found module
        let mut queue: Vec<String> = required_modules.iter().cloned().collect();
        let mut visited = required_modules.clone();

        while let Some(mod_path) = queue.pop() {
            if let Some(file_path) = self.index.modules.get(&mod_path) {
                let content = fs::read_to_string(file_path).unwrap_or_default();
                match syn::parse_file(&content) {
                    Ok(ast) => {
                        let mut local_deps = HashSet::new();
                        self.visit_ast(&ast, &mut local_deps, &mod_path);

                        for dep in local_deps {
                            if !visited.contains(&dep) {
                                visited.insert(dep.clone());
                                required_modules.insert(dep.clone());
                                queue.push(dep);
                            }
                        }
                    }
                    Err(_) => {
                        // Failed to parse, ignore for now
                    }
                }
            } else {
                // No file found for module, ignore for now
            }
        }
        required_modules
    }

    fn visit_ast(&self, ast: &File, collected: &mut HashSet<String>, current_mod: &str) {
        use syn::visit::Visit;
        struct ImportVisitor<'a> {
            collected: &'a mut HashSet<String>,
            current_mod: String,
            index: &'a HashMap<String, PathBuf>,
        }

        impl<'a> Visit<'_> for ImportVisitor<'a> {
            fn visit_use_tree(&mut self, i: &syn::UseTree) {
                // Flatten use tree: use algorithm::io::{Reader, Writer};
                // -> algorithm::io::Reader, algorithm::io::Writer
                // We're interested in prefixes starting with "algorithm".

                // Simplified extraction: just get the full path
                // This is a bit complex with recursive UseTree.
                // For now, let's just cheat and look for paths starting with "algorithm"
                // Actually `syn::UsePath` is what we generally see.

                // We can construct paths during traversal.
                self.traverse(i, String::new());
            }
        }

        impl<'a> ImportVisitor<'a> {
            fn traverse(&mut self, tree: &syn::UseTree, prefix: String) {
                match tree {
                    syn::UseTree::Path(p) => {
                        let new_prefix = if prefix.is_empty() {
                            p.ident.to_string()
                        } else {
                            format!("{}::{}", prefix, p.ident)
                        };
                        self.traverse(&*p.tree, new_prefix);
                    }
                    syn::UseTree::Name(n) => {
                        let full_path = if prefix.is_empty() {
                            n.ident.to_string()
                        } else {
                            format!("{}::{}", prefix, n.ident)
                        };
                        self.check_and_add(&full_path);
                    }
                    syn::UseTree::Rename(n) => {
                        let full_path = if prefix.is_empty() {
                            n.ident.to_string()
                        } else {
                            format!("{}::{}", prefix, n.ident)
                        };
                        self.check_and_add(&full_path);
                    }
                    syn::UseTree::Glob(_) => {
                        // algorithm::io::* -> we should probably include the whole module "algorithm::io"
                        self.check_and_add(&prefix);
                    }
                    syn::UseTree::Group(g) => {
                        for item in &g.items {
                            self.traverse(item, prefix.clone());
                        }
                    }
                }
            }

            fn check_and_add(&mut self, path: &str) {
                // 1. Absolute usage of algorithm
                if path.starts_with("algorithm") {
                    if self.index.contains_key(path) {
                        self.collected.insert(path.to_string());
                    } else {
                        // Check prefixes
                        let parts: Vec<&str> = path.split("::").collect();
                        let mut current = String::new();
                        for (_i, part) in parts.iter().enumerate() {
                            // Fixed unused variable
                            if !current.is_empty() {
                                current.push_str("::");
                            }
                            current.push_str(part);
                            if self.index.contains_key(&current) {
                                self.collected.insert(current.clone());
                            }
                        }
                    }
                } else if !self.current_mod.is_empty() {
                    // 2. Relative usage
                    // e.g. use reader::Reader; inside algorithm::io
                    // candidate: algorithm::io::reader::Reader

                    // Simple logic: try to append path to current_mod
                    // If current_mod is "algorithm::io", path is "reader::Reader".
                    // Full path: "algorithm::io::reader::Reader".
                    // We check if "algorithm::io::reader" is a module.

                    // Also handle `super` and `self`.
                    // But simplified: plain join.
                    let relative_path = format!("{}::{}", self.current_mod, path);
                    // Check if any prefix of relative_path is a known module
                    self.check_if_module_exists(&relative_path);
                }
            }

            fn check_if_module_exists(&mut self, path: &str) {
                // Check if `path` or prefixes match a module in `index`
                // Here `path` is like "algorithm::io::reader::Reader"
                // index has "algorithm::io::reader"

                let parts: Vec<&str> = path.split("::").collect();
                let mut current = String::new();
                for (_i, part) in parts.iter().enumerate() {
                    // Fixed unused variable
                    if !current.is_empty() {
                        current.push_str("::");
                    }
                    current.push_str(part);
                    // println!("  Checking prefix: {}", current);
                    if self.index.contains_key(&current) {
                        self.collected.insert(current.clone());
                        // Keep searching? Maybe nested?
                        // Usually longest match is what we want?
                        // Or we just add the leaf usage string and Bundler figures it out.
                        // Bundler takes "strings" and finds modules.
                        // So if we add "algorithm::io::reader::Reader", Bundler will find "algorithm::io::reader".
                        // Satisfied.
                    }
                }
            }
        }

        let mut visitor = ImportVisitor {
            collected,
            current_mod: current_mod.to_string(),
            index: &self.index.modules,
        };
        visitor.visit_file(ast);
    }
}
