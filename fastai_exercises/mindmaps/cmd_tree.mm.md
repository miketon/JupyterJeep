---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# cmd

## `echo`

### -- state -- 

- ```sh
    echo $?
  ```

  - `==0` : SUCCESS  zero issue
  - `> 0` : FAILED   1 > issues

#### ==[ STDOUT ]==

#### ==[ STDERR ]==

## `tree`

### -- tables --

```sh
tree -L 3
```

#### ==[ Rust ]==

- ```sh
    .                     # on `cargo run` or `cargo build` =>
    ├── Cargo.lock            # [x] exact version of dependencies
    ├── Cargo.toml        # => config dependencies + project info
    ├── src/
    │   └──  main.rs      # => entry point
    └── target/           # => output directory for proj artifacts
        ├── CACHEDIR.TAG      # [x] flags as cache directory
        └── debug/        # => debug artifacts (unopt + db info)
            ├── build/        # compiled dep + build script output
            ├── deps/         # toml crates + dep specific to project
            ├──   echor       # debug binary for project
            ├──   echor.d     # debug symbols for project
            ├── examples/     # compiled examples (if any)
            └── incremental/  # directory to cache incremental build
        └── release/       # => production artifact
    ```

  - `tree -L 3 --du -h`

    - ```sh
        [3.0M]  .
        ├── [3.0K]  Cargo.lock
        ├── [ 188]  Cargo.toml
        ├── [ 238]  err
        ├── [   0]  out
        ├── [1010]  src
        │   └── [ 914]  main.rs
        └── [3.0M]  target
            ├── [ 177]  CACHEDIR.TAG
            └── [3.0M]  debug
                ├── [ 128]  build
                ├── [4.0K]  deps
                ├── [3.0M]  echor
                ├── [  70]  echor.d
                ├── [  64]  examples
                └── [  96]  incremental
      ```

##### `cat`

- -- source --

  - ```sh
    cat src/main.rs
    ```

    - ```sh
        use clap::{
          App, 
          Arg
        }; // import the clap::App struct

        fn main() {
            let matches = App::new("echor") // create new app with the name 'echor' 
                .version("0.1.0") // use semantic version information
                .author("ChillTon <chill.ton@aol.com>") // name and email
                .about("Rust echo") // short description of the program
                .arg(
                    Arg::with_name("text")
                        .value_name("Text")
                        .help("Input text")
                        .required(true)
                        .min_values(1),
                )
                .arg(
                    Arg::with_name("omit_newline")
                        .short("n")
                        .help("Do not print newline")
                        .takes_value(false),
                )
                .get_matches(); // Tell the `App` to parse the arguments
            
            println!("{:#?}", matches);
        }
      ```

##### `cargo`

- -- run --

  - ```sh
    cargo run -- --help
    ```

    - ```sh
        echor 0.1.0
        ChillTon <chill.ton@aol.com>
        Rust echo

        USAGE:
            echor [FLAGS] <Text>...

        FLAGS:
            -h, --help       Prints help information
            -n               Do not print newline
            -V, --version    Prints version information

        ARGS:
            <Text>...    Input text
      ```

