---
markmap:
  colorFreezeLevel: 2
  maxWidth: 400
---

# Solution

## [Ch 7 : File Structure](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### Overview

- Rust's File Structure
  - **[ Goals ]**
    - **code organization**
      - // minimize **commit** collision
    - **scope control**
      - // minimize **string** collision
    - effective **path usage**
      - // maximize **import clarity**

- Summary of **File Structure**
  - **[ Packages ]** as a way to build, test, and share **crates**
  - **[ Crates ]** as a **Tree of Modules** that produce:
    - **library** or
    - **executable**
  - Recap of the **[ Module System ]**:
    - **features** and
    - **concepts** discussed

## `Project`

### **[ CRATE ]**

#### **[ Root ]**

- Is the **Anchor Point** for the **Rust compiler**
- Understanding the package structure
  - `Cargo.toml`
    - includes
      - external **[dependencies]**
    - configuration
      - version
      - **[ flags ]**
        - feature
        - build
  - `src` directory
    - `main.rs`
    - `lib.rs`
- **Project Type**:
  - library crates
  - binary crates

### Source **Files**

#### **Grouping** Code

- File + Folder Organization
  - **readability**
  - **reuse**
  - **scope**
    - // explicit
      - **privacy** and
      - making items **public**

#### **[ Modules ]**

##### Defined **NameSpace**

- **import** { -- /paths -- }
  - The `use` keyword
    - brings path **address into scope**
    - **simplifies** item path **address**
- { -- code block -- }
  - `mod` declarations

    -

      ```rust
        pub mod greetings {
          pub fn hello() {
            println!("Hello World");
          }
        }
      ```

  - `pub` => public

##### **[ Module Tree ]**

- Supports Composition
  - Split **modules** into smaller **files**
  - Example
    - Restaurant
      - front of house
        - // client handle for menu, order, payment info
      - back of house
        - // backend server for cooking, supply, storage
- Cross Access Modules
  - use **[ Paths ]**

#### **[ Paths ]**

##### Explicit

- `use` paths
  - idiomatic **Group**
    - **Glob** Operator `*`
      - Brings all public items from a module into scope

        -

          ```rust
            use bevy::prelude::*;
          ```

    - **Nested** Paths
      - Using nested paths to
        - improve readability
          - reduce repetition

        -

          ```rust
            use bevy_ecs_tilemap::{
              helpers::square_grid::neighbors::Neighbors,
              prelude::*,
            };
          ```

  - across **File/Folder**
    - `Absolute`
      - from the `crate root`
    - `Relative`
      - from the `current module`
        - // namespace
          - **self**
          - **super**
          - **identifier**

      -

        ```rust
          // Relative import using self
          use self::greetings::hello;
        ```

    - **Alternate**
      - Different file path options for module code organization
        - @todo ???
      - Understanding the file naming conventions for modules
        - @todo ???
      - Choosing between different file path styles based on project structure
        - @todo ???

## External `Package`

- Using External Packages
  - root **directory**
    - `cargo.toml`
      - Add external packages as **[dependencies]** here

        -

          ```rust
            [dependencies]
            bevy_ecs_tilemap = "0.10.0"
          ```

  - **files**
    - `src`/main.rs/lib.rs
      - // must list any **mods**
        - @todo : add example code

    - source.rs **file**
      - `use` keyword
        - Bring items from external packages into scope
          - standard library
            - // included implicitly with Rust installation
          - [dependencies]
            - // **cargo.toml** includes
