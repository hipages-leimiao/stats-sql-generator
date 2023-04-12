# sql generator for profile stats

## Setup

```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# run
## option 1: (build then run)
cargo build --release
target/release/profile_stats {run|parse}

## option 2: (build and run directly)
cargo run -- {run|parse}
```

## Run

### help

```trycmd
$ profile_stats run --help
Usage: profile_stats run [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>
          
  -s, --s-type <S_TYPE>
          [default: monthly] [possible values: weekly, monthly, quarterly]
  -k, --key <KEY>
          [default: "[..] - [..]"]
  -m, --migration-file-name <MIGRATION_FILE_NAME>
          [default: SeedProfileStatsBatch[..]]
  -d, --do-filter
          
  -r, --raise-pr
          
  -h, --help
          Print help

```

### Example

```trycmd
$ profile_stats run -f fixtures/test.csv -s monthly -m SeedProfileStatsBatch4
migration sql has been generated to file: migration_output.php

```

## Parse (interactive input)

### Example

```shell
$ profile_stats parse
✔ Path of stats xlsx · fixtures/test.csv
✔ Select a stats type · Monthly: 1 Feb - 28 Feb 2023
✔ Filename of this migration · SeedProfileStatsBatch
✔ Auto raise phinx migration PR? · false
migration sql has been generated to file: migration_output.php

```
