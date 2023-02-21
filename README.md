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
Usage: stats-sql-generator run [OPTIONS] --file <FILE> --key <KEY> --migration-file-name <MIGRATION_FILE_NAME>

Options:
  -f, --file <FILE>                                
  -p, --parsed                                     
  -k, --key <KEY>                                  
  -m, --migration-file-name <MIGRATION_FILE_NAME>  
  -h, --help                                       Print help

```

### Example

source data require parse

```trycmd
$ stats-sql-generator run -f fixtures/test.xlsx -k "1 September 2022 - 31 January 2023" -m SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```

source data already parsed
```trycmd
$ stats-sql-generator run -f fixtures/test.csv -p -k "1 September 2022 - 31 January 2023" -m SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```

## Parse (interactive input)

### Example

```shell
$ stats-sql-generator parse
✔ Path of stats xlsx · fixtures/test.xlsx
✔ Is data already parsed? · true
✔ Time range for this batch of stats migration · 1 September 2022 - 31 January 2023
✔ Filename of this migration · SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```
