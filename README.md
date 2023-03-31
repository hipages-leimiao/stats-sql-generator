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
Usage: stats-sql-generator run [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>
          
  -s, --s-type <S_TYPE>
          [default: default] [possible values: default, weekly, monthly, quarterly]
  -k, --key <KEY>
          [default: "1 September 2022 - 28 February 2023"]
  -m, --migration-file-name <MIGRATION_FILE_NAME>
          [default: SeedProfileStatsBatch]
  -r, --raise-pr
          
  -h, --help
          Print help

```

### Example

```trycmd
$ stats-sql-generator run -f fixtures/test.csv -s monthly -m SeedProfileStatsBatch4
migration sql generates to file: migration_output.php

```

## Parse (interactive input)

### Example

```shell
$ stats-sql-generator parse
✔ Path of stats xlsx · fixtures/test.csv
✔ Select a stats type · Monthly: 1 Feb - 28 Feb 2023
✔ Filename of this migration · SeedProfileStatsBatch
✔ Auto raise phinx migration PR? · false
migration sql generates to file: migration_output.php

```
