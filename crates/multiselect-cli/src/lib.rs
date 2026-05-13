mod model;
mod tui;

pub use model::Item;

use std::io::Read;

pub struct Multiselect {
    prompt: String,
    items: Vec<Item>,
}

impl Multiselect {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            items: Vec::new(),
        }
    }

    pub fn items(mut self, items: Vec<Item>) -> Self {
        self.items = items;
        self
    }

    pub fn run(self) -> Result<Option<Vec<String>>, String> {
        let tree = model::Tree::build(self.items)?;
        tui::run(&self.prompt, tree)
    }
}

fn print_help() {
    println!("Usage: multiselect [--prompt <text>] < items.tsv");
    println!();
    println!("Reads tab-separated items from stdin, displays an interactive");
    println!("multiselect TUI, and prints selected leaf ids to stdout (one per line).");
    println!();
    println!("Each input line: id<TAB>label<TAB>parent<TAB>selected");
    println!("  label    optional, defaults to id");
    println!("  parent   optional, empty for top-level");
    println!("  selected optional, '1'/'true'/'yes' for pre-selected");
    println!();
    println!("Exit codes: 0 confirmed, 1 cancelled, 2 invalid usage.");
}

pub fn multiselect_main() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut prompt = String::from("Select items:");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--prompt" | "-p" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("--prompt requires a value");
                    return 2;
                };
                prompt = v.clone();
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                return 0;
            }
            other => {
                eprintln!("unknown argument: {}", other);
                return 2;
            }
        }
    }

    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {}", e);
        return 1;
    }

    let mut items = Vec::new();
    for (lineno, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('\t');
        let id = parts.next().unwrap_or("").to_string();
        if id.is_empty() {
            eprintln!("line {}: empty id", lineno + 1);
            return 1;
        }
        let label = parts
            .next()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| id.clone());
        let parent_str = parts.next().unwrap_or("");
        let parent = if parent_str.is_empty() {
            None
        } else {
            Some(parent_str.to_string())
        };
        let selected = matches!(parts.next().unwrap_or(""), "1" | "true" | "yes" | "y");
        items.push(Item {
            id,
            label,
            parent,
            selected,
        });
    }

    match Multiselect::new(prompt).items(items).run() {
        Ok(Some(ids)) => {
            for id in ids {
                println!("{}", id);
            }
            0
        }
        Ok(None) => 1,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    }
}
