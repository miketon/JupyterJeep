---
markmap:
  colorFreezeLevel: 2
  maxWidth: 400
---

# Solution

## [Ch 7 : File Structure](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### Intro

- Overview of Rust's file structure
  - **Importance** of:
    - **code organization**
    - **scope control**
    - effective **path usage**

- Summary of **File Structure**
  - **Packages** as a way to build, test, and share crates
  - **Crates** as a **tree of modules** that produce:
    - **library** or
    - **executable**
  - Recap of the **module system**:
    - **features** and
    - **concepts** discussed

### Packages

- Understanding Packages as a method to :
  - **crates::**
    - **build**
    - **test**
    - **share**

### Crates

- Are a **tree of modules** resulting in :
  - **library** or
  - **executable**

### Module System

- Recap of the module system: features and concepts

## Project

### Grouping Code

- Strategies for code organization within a :
  - **crate::**
    - **readability**
    - **reuse**
- Controlling: 
  - **privacy** and
  - making items **public**

### Crate Root

- Defining the Crate root: the starting point for the Rust compiler
- Understanding the package structure:
  - `Cargo.toml`
  - `src` directory
- **Project Type**:
  - library crates
  - binary crates

### Code Organization

#### Modules

- Understanding Modules: Organizing code and controlling privacy
- Use of the 'use' keyword to bring paths into scope and simplify item access

##### Module Tree

- Splitting modules into separate files for better organization
  - Using mod declarations and file organization to define module code
  - Making modules and items public to enable access from other modules
- Understanding paths for accessing items in modules
  - Absolute and relative paths for accessing items in modules
  - Re-exporting names with `pub use`
- Example:
  - Structuring code based on a restaurant's:
    - front of house
    - back of house

#### Paths

- Naming items in modules through paths
- Understanding absolute and relative paths
  - `Absolute` paths starting from the `crate root`
  - `Relative` paths starting from the `current module`
- Best practices for creating idiomatic 'use' paths
  - Nested Paths
    - Using nested paths to reduce repetition and improve readability
  - Glob Operator
    - Bringing all public items from a module into scope using the glob operator

## External Package

- Using External Packages
  - Adding external packages as dependencies in Cargo.toml
  - Bringing items from external packages into scope using use statements
  - Using the standard library and specifying paths to bring items into scope

- Alternate File Paths
  - Different file path options for module code organization
  - Understanding the file naming conventions for modules
  - Choosing between different file path styles based on project structure
