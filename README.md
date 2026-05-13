# multiselect-cli

A Rust library and CLI tool for an interactive hierarchical multi-select picker. Reads items from stdin as tab-separated values, displays a TUI for selection, and writes selected leaf IDs to stdout.

## Features

- **Hierarchical items** — parent/child relationships defined via a `parent` column; toggling a parent toggles all descendants
- **Pre-selected items** — mark items as pre-selected in the input
- **TUI on `/dev/tty`** — keeps stdin/stdout clean for shell pipelines; the picker draws on the controlling terminal while stdout receives only the selected IDs
- **Library API** — use `Multiselect::new(prompt).items(items).run()` to embed the picker in another Rust program

## CLI Usage

The `multiselect` binary reads tab-separated items from stdin:

```sh
multiselect [--prompt <text>] < items.tsv
```

Each input line has up to four tab-separated fields:

```
id<TAB>label<TAB>parent<TAB>selected
```

- `id` — required, must be unique
- `label` — optional display text, defaults to `id` if empty
- `parent` — optional, references another item's `id`; empty for top-level
- `selected` — optional, set to `1`, `true`, `yes`, or `y` to pre-select

Output: selected leaf IDs (one per line) on stdout.

Exit codes: `0` confirmed, `1` cancelled or error, `2` invalid usage.

Example:

```sh
printf 'a\tAlpha\t\t\nb\tBeta\t\t1\nc\tGamma\t\t\n' | multiselect --prompt "Pick one:"
```

Key bindings: `↑`/`↓` (or `j`/`k`) navigate, `enter` to select, `s` to submit, `esc`/`q`/`Ctrl-C` to exit.

## Library Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
multiselect-cli = { git = "https://github.com/Artemis-Cooperative/multiselect-cli.git", tag = "v1.0.0" }
```

Then use the builder API:

```rust
use multiselect_cli::{Item, Multiselect};

let items = vec![
    Item { id: "a".into(), label: "Alpha".into(), parent: None, selected: false },
    Item { id: "b".into(), label: "Beta".into(),  parent: None, selected: true  },
];

match Multiselect::new("Pick one:").items(items).run()? {
    Some(ids) => println!("selected: {:?}", ids),
    None      => println!("cancelled"),
}
```

`Multiselect::run()` returns `Result<Option<Vec<String>>, String>`:

- `Ok(Some(ids))` — user confirmed; `ids` are the selected leaf IDs in render order
- `Ok(None)` — user cancelled
- `Err(msg)` — input validation or terminal error
