In Jupyter all rust code is in "static" scope
- implicitly we need to wrap each cell in it's own {} block
- {} per cell == main.rs main function
- global functions we can leave at "static" scope

Else we will get this error :

Error: The variable `r2` contains a reference with a non-static lifetime so
can't be persisted. You can prevent this error by making sure that the
variable goes out of scope - i.e. wrapping the code in {}.


// To generate assembly, run this in cmd line :

rustc --emit=llvm-ir,asm src/main.rs

// or

rustc --emit asm src/main.rs -C "llvm-args=-x86-asm-syntax=intel" -o hello_rust.s

// This command generates an assembly file named `hello_rust.s` in the same  
// folder as your source code. The `-C "llvm-args=-x86-asm-syntax=intel"` option  
// is used to set the assembly syntax to Intel, which is easier to read than the  
// default AT&T syntax.