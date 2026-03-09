use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::process::Command;

pub struct Cleaner {
    edition: String,
}

#[derive(Debug)]
struct Warning {
    line: usize, // 1-based
    name: String,
    kind: WarningKind,
}

#[derive(Debug, Clone)]
enum WarningKind {
    UnusedImport,
    /// Top-level item (struct, trait, enum, free function, type alias, constant)
    UnusedTopLevelItem,
}

impl Cleaner {
    pub fn new(edition: &str) -> Self {
        Self {
            edition: edition.to_string(),
        }
    }

    pub fn clean(&self, code: &str) -> String {
        let mut current = code.to_string();

        for _ in 0..20 {
            let warnings = self.get_warnings(&current);
            if warnings.is_empty() {
                break;
            }

            let new_code = self.remove_items(&current, &warnings);
            if new_code == current {
                break;
            }

            current = new_code;
        }

        Self::remove_empty_modules(&current)
    }

    /// Convert the top-level `pub mod algorithm` to `mod algorithm` so rustc's dead_code
    /// lint fires on pub items inside. Inner `pub mod` are preserved to keep access chains
    /// working. The line count stays identical so warning line numbers map back correctly.
    fn strip_top_level_pub_mod(code: &str) -> String {
        let re = Regex::new(r"(?m)^pub mod algorithm\b").unwrap();
        re.replace(code, "mod algorithm").to_string()
    }

    fn run_rustc(&self, code: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("_bundler_clean_tmp.rs");
        fs::write(&temp_path, code).unwrap();

        let out_path = temp_dir.join("_bundler_clean_tmp_out");

        let output = Command::new("rustc")
            .args([
                "--edition",
                &self.edition,
                "-W",
                "dead_code",
                "-o",
                out_path.to_str().unwrap(),
            ])
            .arg(temp_path.to_str().unwrap())
            .output();

        match output {
            Ok(o) => String::from_utf8_lossy(&o.stderr).to_string(),
            Err(_) => String::new(),
        }
    }

    fn compiles_ok(&self, code: &str) -> bool {
        let stderr = self.run_rustc(code);
        !stderr.contains("aborting due to")
    }

    fn get_warnings(&self, code: &str) -> Vec<Warning> {
        let analysis_code = Self::strip_top_level_pub_mod(code);
        let stderr = self.run_rustc(&analysis_code);

        // If there are hard errors, don't try to clean
        if stderr.contains("aborting due to") {
            return vec![];
        }

        Self::parse_warnings(&stderr)
    }

    fn parse_warnings(stderr: &str) -> Vec<Warning> {
        let mut warnings = Vec::new();
        let blocks: Vec<&str> = stderr.split("warning: ").collect();

        // Only match top-level items (struct, trait, enum, free function, type alias, constant)
        // Skip "method" and "associated function" — those are inside impl/trait blocks
        // and removing individual methods breaks the surrounding block.
        let item_re = Regex::new(
            r"^(function|struct|trait|enum|type alias|constant) `([^`]+)` is never (?:used|constructed|read)",
        )
        .unwrap();
        let import_re = Regex::new(r"^unused import: `([^`]+)`").unwrap();
        let line_re = Regex::new(r"--> .*:(\d+):\d+").unwrap();

        for block in blocks.iter().skip(1) {
            let first_line = block.lines().next().unwrap_or("").trim();

            let (kind, name) = if let Some(caps) = import_re.captures(first_line) {
                (WarningKind::UnusedImport, caps[1].to_string())
            } else if let Some(caps) = item_re.captures(first_line) {
                (WarningKind::UnusedTopLevelItem, caps[2].to_string())
            } else {
                continue;
            };

            for line in block.lines().skip(1) {
                if let Some(caps) = line_re.captures(line) {
                    let line_num: usize = caps[1].parse().unwrap();
                    warnings.push(Warning {
                        line: line_num,
                        name,
                        kind,
                    });
                    break;
                }
            }
        }

        warnings
    }

    /// Compute the set of line indices to remove for a single warning.
    fn removal_set_for_warning(lines: &[&str], w: &Warning) -> (HashSet<usize>, HashSet<String>) {
        let mut remove_set = HashSet::new();
        let mut removed_types = HashSet::new();
        let idx = w.line.saturating_sub(1);
        if idx >= lines.len() {
            return (remove_set, removed_types);
        }

        match w.kind {
            WarningKind::UnusedImport => {
                remove_set.insert(idx);
            }
            WarningKind::UnusedTopLevelItem => {
                let (start, end) = Self::item_range(lines, idx);
                for i in start..=end {
                    remove_set.insert(i);
                }
                removed_types.insert(w.name.clone());
            }
        }

        // Also remove impl blocks for removed types
        for type_name in &removed_types.clone() {
            let pattern = format!(
                r"^\s*(pub\s+)?impl\b[^{{]*\b{}\b",
                regex::escape(type_name)
            );
            if let Ok(re) = Regex::new(&pattern) {
                for (i, line) in lines.iter().enumerate() {
                    if re.is_match(line) && !remove_set.contains(&i) {
                        let (start, end) = Self::item_range(lines, i);
                        for j in start..=end {
                            remove_set.insert(j);
                        }
                    }
                }
            }
        }

        (remove_set, removed_types)
    }

    fn apply_removal(lines: &[&str], remove_set: &HashSet<usize>) -> String {
        lines
            .iter()
            .enumerate()
            .filter(|(i, _)| !remove_set.contains(i))
            .map(|(_, l)| *l)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn remove_items(&self, code: &str, warnings: &[Warning]) -> String {
        let lines: Vec<&str> = code.lines().collect();

        // First, try removing everything at once
        let mut total_remove_set = HashSet::new();
        for w in warnings {
            let (set, _) = Self::removal_set_for_warning(&lines, w);
            total_remove_set.extend(set);
        }
        let candidate = Self::apply_removal(&lines, &total_remove_set);
        if self.compiles_ok(&candidate) {
            return candidate;
        }

        // If that fails, try each warning individually and keep only safe removals
        let mut safe_remove_set: HashSet<usize> = HashSet::new();
        for w in warnings {
            let (set, _) = Self::removal_set_for_warning(&lines, w);
            let mut trial_set = safe_remove_set.clone();
            trial_set.extend(&set);
            let candidate = Self::apply_removal(&lines, &trial_set);
            if self.compiles_ok(&candidate) {
                safe_remove_set = trial_set;
            }
        }

        Self::apply_removal(&lines, &safe_remove_set)
    }

    fn item_range(lines: &[&str], decl_line: usize) -> (usize, usize) {
        // Look backwards for attributes
        let mut start = decl_line;
        while start > 0 {
            let prev = lines[start - 1].trim();
            if prev.starts_with("#[")
                || prev.starts_with("///")
                || prev.starts_with("//!")
                || (prev.ends_with(']') && !prev.contains('='))
            {
                start -= 1;
            } else {
                break;
            }
        }

        let trim = lines[decl_line].trim();

        // One-liner
        if trim.ends_with(';') && !trim.contains('{') {
            return (start, decl_line);
        }

        // Find matching brace with a state machine that skips strings/chars/comments
        let mut depth = 0i32;
        let mut found_brace = false;

        for i in decl_line..lines.len() {
            let mut chars = lines[i].chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '"' => {
                        while let Some(sc) = chars.next() {
                            if sc == '\\' {
                                chars.next();
                            } else if sc == '"' {
                                break;
                            }
                        }
                    }
                    '\'' => {
                        if chars.peek().is_some() {
                            let next = chars.next().unwrap();
                            if next == '\\' {
                                chars.next();
                            }
                            if chars.peek() == Some(&'\'') {
                                chars.next();
                            }
                        }
                    }
                    '/' => {
                        if chars.peek() == Some(&'/') {
                            break;
                        } else if chars.peek() == Some(&'*') {
                            chars.next();
                            loop {
                                match chars.next() {
                                    Some('*') if chars.peek() == Some(&'/') => {
                                        chars.next();
                                        break;
                                    }
                                    None => break,
                                    _ => {}
                                }
                            }
                        }
                    }
                    '{' => {
                        depth += 1;
                        found_brace = true;
                    }
                    '}' => {
                        depth -= 1;
                    }
                    _ => {}
                }

                if found_brace && depth == 0 {
                    return (start, i);
                }
            }
        }

        (start, decl_line)
    }

    fn remove_empty_modules(code: &str) -> String {
        // Remove single-line empty modules
        let re = Regex::new(r"(?m)^\s*pub\s+mod\s+\w+\s*\{\s*\}\s*\n?").unwrap();
        let mut result = code.to_string();
        loop {
            let new_result = re.replace_all(&result, "").to_string();
            if new_result == result {
                break;
            }
            result = new_result;
        }

        // Remove multi-line empty modules (with only whitespace inside)
        let multiline_re =
            Regex::new(r"(?ms)^\s*pub\s+mod\s+\w+\s*\{\s*\n\s*\}\s*\n?").unwrap();
        loop {
            let new_result = multiline_re.replace_all(&result, "").to_string();
            if new_result == result {
                break;
            }
            result = new_result;
        }

        result
    }
}
