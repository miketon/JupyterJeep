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
  - ==[ io ]== from **echo**
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

- ==[ **cargo** run ]==
  - `main.rs`
    - ==[ io ]== from **echor**
      - `use clap::{}`
      - `fn main(){}`
        - // instantiate App
          - `App::new("echor")`
        - // get matches
          - // get text
          - // get new line flag (omit)
        - // echo arguments from echor

## [ binary ]

### target/

- debug/
  - ==[ cargo ]== **echor**
    - `cargo run -- [n] args...`
      - // generated on build or run

### tests/

- `cli.rs`
  - ==[ io test ]== **echor** == **echo** ???
    - // imports
      - `use`
        - `assert_cmd::Commands;`
          - // run system binaries and assert on their output
        - `predicates::prelude::*;`
          - // predicate crate is used for writing test assertions
        - `std::fs;`
    - // alias
    - // helper functions
    - // echo == echor io ?
      - // no args
      - // args

#### expected/

- ==[ io text **files** ]==
  - `bash mk-outs.sh`

    - ```sh
      # default
        args_1.txt
        args_2.txt
      # no newline
        args_1.n.txt
        args_2.n.txt
      ```