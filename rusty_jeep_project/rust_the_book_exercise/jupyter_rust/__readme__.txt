In Jupyter all rust code is in "static" scope
- implicitly we need to wrap each cell in it's own {} block
- {} per cell == main.rs main function
- global functions we can leave at "static" scope

Else we will get this error :

Error: The variable `r2` contains a reference with a non-static lifetime so
can't be persisted. You can prevent this error by making sure that the
variable goes out of scope - i.e. wrapping the code in {}.