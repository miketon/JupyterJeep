---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# RUST 🚧 = ==[ UNSAFE ]== = 🚧

## 2nd **| Hidden Language |**

- DOES NOT **enforce MEMORY SAFETY**

## -- Why Does it Exist --

### ==[ HARDWARE ]==

- Inherently 🚧 Unsafe 🚧
  - Potentially Higher Performance
  - Directly manipulating Memory

    - ```rust
      {
        let address = 0x012345usize;
        let r = address as *const i32; 
      }
      ```

      - Usually there is NO GOOD reason to
       write code like this.. but is possible
      - Trying to use abitrary memory is **UNDEFINED**
        - there might be data at that address or not
        - compiler may have optimized out the data, no memory access
        - or program might error with **segmentation fault**

### ==[ STATIC ANALYSIS]==

- Inherently Conservative
  - Code will be **REJECTED by DEFAULT**
    - if RUST Compiler does NOT have
     ENOUGH INFORMATION
    - **"Trust me, I Know What I'm Doing"**
      - you are TAKING LIABILITY for Memory Artifacts

## -- SUPER POWERS --

### | ACTIVATE |

- -- use 🚧 **==[ unsafe ]==** **keyword** --
- -- **Isolate** 🚧 unsafe Code --
  - start a 🧰 **new block**
  - to hold **unsafe** code

### | 5 **POWERS** |

#### 1. **Dereference** a **raw** pointer

- ==[ raw pointer ]==
  - **differences** vs -- references --
    1. allowed to **ignore borrowing rules**
    2. having both **mutable** and **immutable** pointers
    3. **multiple mutable** pointers to the **same address**
    4. allowed to be **null**
    5. no **automatic cleanup**

  - `*const T`
    - -- immutable --
      - pointer **can NOT** be directly **assigned**
      after getting **dereferenced**
  - `*mut T`
    - -- mutable --
  - | EXAMPLE |

    - **Create Raw Pointer** in
      **safe** code **OK**

      - ```rust
          let mut num = 5;
          let r1 = &num as *const i32;
          let r2 = &mut num as *mut i32;
        ```

        - `*` is part of the Type name
          not a **dereference** operator
        - **CREATING** raw pointers does **NO HARM**

    - **Dereference MUST** be
      in a **unsafe** code block

      - ```rust
          unsafe {
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2)
          }
        ```

        - **DEREFERENCING** pointers is where
        we **risk** getting **INVALID values**

#### 2. Call an **unsafe** function or method

- ==[ **unsafe** function ]==
  
  - ```rust
      unsafe fn dangerous() {}
    ```

    - unsafe functions **defined in safe code OK**

  - ```rust
      unsafe {
        dangerous();          
      }
    ```

    - but **MUST be called** in an **unsafe code block**

#### 3. Access or modify a **mutable static** variable

#### 4. Implement an **unsafe trait**

#### 5. Access **fields of unions**

- the **Compiler will NOT check** that
  we are reading the **correct** field
  - | EXAMPLE |

    - ```rust
        union Number {
          integer : i32,
          float : f32,
        }

        {
          let num = Number { integer: 42 };

          unsafe { // prints 42
            println!("num as integer: {}", num.integer); 
          }

          unsafe { // undefined behavior
            println!("num as float: {}", num.float);
          }
        }

      ```

      - num = Number { **integer**: 42 }
      - num.**integer** -> prints **42**
        - // num as integer: 42
      - num.**float** -> prints **undefined** value
        - // num as float: 0.000000000000000000000000000000000000000000059
