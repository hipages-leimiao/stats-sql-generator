# sql generator for profile stats

## Setup

```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# run
## option 1: (build then run)
cargo build --release
target/release/stats-sql-generator {run|parse}

## option 2: (build and run directly)
cargo run -- {run|parse}
```

## Run

### help

```trycmd
$ stats-sql-generator run --help
Usage: stats-sql-generator run --file <FILE> --key <KEY> --migration-file-name <MIGRATION_FILE_NAME>

Options:
  -f, --file <FILE>
  -k, --key <KEY>
  -m, --migration-file-name <MIGRATION_FILE_NAME>
  -h, --help                                       Print help

```

### Example

```trycmd
$ stats-sql-generator run -f fixtures/test.xlsx -k "1 September 2022 - 31 January 2023" -m SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```

## Parse (interactive input)

### Example

```shell
$ stats-sql-generator parse
✔ Path of stats xlsx · fixtures/test.xlsx
✔ Time range for this batch of stats migration (eg: 1 September 2022 - 31 January 2023) · 1 September 2022 - 31 January 2023
✔ Filename of this migration (eg: SeedProfileStatsBatch*) · SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```
