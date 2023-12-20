---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# echor

## ./

- `Cargo.toml`
  - [package]
    - // metadata for app

      - ```sh
          name = "echor"
          version = "0.1.0"
          edition = "2021"
        ```

  - [dependencies]
    - // crates for `distribution` binary

      - ```sh
          clap = "2.33"
        ```

  - [dev-dependencies]
    - // crates for `development` work

      - ```sh
          assert_cmd = "2"
          predicates = "2"
        ```

- `mk-outs.sh`
  - // bash script to generate files for `expected` test string

    - `!/usr/bin/env bash`
      - // shebang : os env to execute bash commands
    - `OUTDIR="tests/expected"`
      - // output directory where we will generate test files
      - `[[ ! -d "$OUTDIR" ]] && mkdir -p $OUTDIR`
        - // mkdir if one doesn't already exist
    - `echo "Hello there" > $OUTDIR/args_1.txt`
      - ...
        - // print expected output and save to file
        - // we will unit test 'echor' binary output against these files!

## src/

- `main.rs`
  - `use clap::{}`
  - `fn main(){}`

## [ binary ]

### target/

- debug/
  - ==[ echor ]==

### tests/

- `cli.rs`

#### expected/
