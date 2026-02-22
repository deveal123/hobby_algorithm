use crate::resolver::{ModuleIndex, Resolver};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

pub struct Bundler {
    resolver: Resolver,
    index: ModuleIndex,
}

impl Bundler {
    pub fn new(index: ModuleIndex) -> Self {
        Bundler {
            resolver: Resolver::new(ModuleIndex {
                modules: index.modules.clone(),
            }),
            index,
        }
    }

    pub fn bundle(&self, input_path: &Path) -> String {
        let required_paths = self.resolver.resolve(input_path);

        // We have a list of usage strings like "algorithm::io::Reader".
        // We need to map these to module paths.
        // "algorithm::io::Reader" -> "algorithm::io" (if Reader is in io)
        // or "algorithm::io::reader" (if we need inner).

        // Strategy:
        // 1. Identify all modules that need to be explicitly defined.
        //    Any module in `self.index` that is a prefix of a `required_path` OR key in `dependencies`.
        //    Actually `resolve` returns strings found in `use`.
        //    Real logical dependencies are calculated.

        // Filter required_paths to those that match valid modules in index.
        let mut modules_to_include = HashSet::new();
        // Always include root "algorithm" if anything is used
        modules_to_include.insert("algorithm".to_string());

        for req in &required_paths {
            // Find longest prefix that is a module
            let parts: Vec<&str> = req.split("::").collect();
            let mut current = String::new();
            for part in parts {
                if !current.is_empty() {
                    current.push_str("::");
                }
                current.push_str(part);
                if self.index.modules.contains_key(&current) {
                    modules_to_include.insert(current.clone());
                }
            }
        }

        // No automatic recursive inclusion.
        // We rely on resolver to find everything that is USED.

        let final_modules = modules_to_include;

        // Now construct the output.
        // We assume "algorithm" is the crate root.

        // Read input file
        let original_code = fs::read_to_string(input_path).unwrap();
        // Remove "use algorithm::...;" lines? Or keep them?
        // If we define "mod algorithm { ... }", then "use algorithm::..." works if "algorithm" is top level mod.
        // But "algorithm" is a crate name usually. In single file, it becomes a module.
        // `use algorithm::io::Reader;` works if `mod algorithm` exists at top level.

        // We will restructure modules into a tree.
        // algorithm
        //   io
        //     reader
        //     writer

        let mut output = String::new();

        // Append modules
        // We need to generate nested modules.
        // Use a BTreeMap to sort by name for deterministic output.
        // Map: "algorithm::io" -> content

        let mut code_map: BTreeMap<String, String> = BTreeMap::new();
        for mod_name in &final_modules {
            if let Some(path) = self.index.modules.get(mod_name) {
                let content = fs::read_to_string(path).unwrap();
                // We need to remove "mod xxx;" lines because we will inline them?
                // Or we keep them and allow `mod xxx { content }` syntax next to it?
                // Rust doesn't allow `mod xxx; mod xxx { ... }`.
                // So we must remove `mod xxx;` lines from the content.

                let mut filtered_lines = Vec::new();
                for line in content.lines() {
                    let trim = line.trim();
                    if (trim.starts_with("mod ") || trim.starts_with("pub mod "))
                        && trim.ends_with(";")
                        && !trim.contains("{")
                    {
                        // Always remove `mod xxx;` lines.
                        // If it's a required module, it will be re-added as `mod xxx { ... }` by `append_submodules`.
                        // If it's not required, we want to drop it entirely.
                        continue;
                    } else {
                        filtered_lines.push(line);
                    }
                }
                code_map.insert(mod_name.clone(), filtered_lines.join("\n"));
            }
        }

        // Reconstruct tree
        // "algorithm" is root.
        // "algorithm::io" is child.

        if let Some(root_content) = code_map.get("algorithm") {
            output.push_str("pub mod algorithm {\n");
            output.push_str(root_content);
            output.push_str("\n");

            // Recursively append children
            // This is tricky with string concatenation.
            // Better: define a recursive function to build the tree.
            // Or, simply parsing "algorithm" content is seemingly hard to inject children into right place.
            // BUT, Rust allows `mod io;` and then supplying it.
            // Actually, `mod io { ... }` REPLACES `mod io;`.
            // So, we removed `mod io;` lines given we are inlining them.
            // Thus we can just append `pub mod io { ... }` at the end of `algorithm` block?
            // YES.

            Self::append_submodules("algorithm", &final_modules, &code_map, &mut output);

            output.push_str("}\n");
        }

        output.push_str("\n");
        output.push_str("// Original algorithm code is in https://github.com/deveal123/hobby_algorithm\n")
        output.push_str(&original_code);

        output
    }

    fn append_submodules(
        parent_mod: &str,
        final_modules: &HashSet<String>,
        code_map: &BTreeMap<String, String>,
        output: &mut String,
    ) {
        // Find direct children of parent_mod in final_modules
        // e.g. parent="algorithm", child="algorithm::io"

        for mod_name in final_modules {
            // check if mod_name is direct child
            if let Some(suffix) = mod_name.strip_prefix(parent_mod) {
                if suffix.starts_with("::") {
                    let rest = &suffix[2..];
                    if !rest.contains("::") && !rest.is_empty() {
                        // Direct child found
                        let child_name = rest;
                        output.push_str(&format!("pub mod {} {{\n", child_name));
                        if let Some(content) = code_map.get(mod_name) {
                            output.push_str(content);
                            output.push_str("\n");
                            Self::append_submodules(mod_name, final_modules, code_map, output);
                        }
                        output.push_str("}\n");
                    }
                }
            }
        }
    }
}

// Helper to satisfy borrow checker in new()
struct MockCloneIndex<'a>(&'a ModuleIndex);
impl<'a> From<MockCloneIndex<'a>> for ModuleIndex {
    fn from(m: MockCloneIndex<'a>) -> Self {
        ModuleIndex {
            modules: m.0.modules.clone(),
        }
    }
}
