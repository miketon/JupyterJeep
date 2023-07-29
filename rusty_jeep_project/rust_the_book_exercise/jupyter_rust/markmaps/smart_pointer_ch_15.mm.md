---
markmap:
   colorFreezeLevel: 2
   maxWidth: 500
---

# Smart Pointers

## **[ HEAP ]**

### **-- POINTER --**

#### | explain |

- concept
  - variable containing an **address** in memory
  - this address **points** at some **data**
  - **deref** a pointer to **follow** to it's **value**
    - `coercion`
      - **OK**::mutable   ref => immutable ref
      - **NO**::immutable ref => mutable ref
- **[ Borrow Rules ]**
  - **smart pointer OWN** data
    - **resource manage**
      - responsible for **clean up** when **out of scope**
    - **control currency**
      - managing **thread** access
        - **Refcell** v **ArCell**
        - reference v atomic
    - **enforce variant**
      - enforce condition on data
      - allows **interior mutability**
      - **check @runtime** v @compiletime
    - **support complex structures**
      - such as linked lists, **trees**, **graphs** ... etc
  - references only BORROW
    - @audit : Clarify what this matters???

#### | usage |

- (1) **allocate**
  - fixed **size**
    - independent of data it's pointing to
  - enables **[Indirection]** (recursion)
- (2) update **address**
  - **FLASH** transfer **[Ownership]**
  - Does **NOT** copy LARGE amounts of Data
- (2) **deallocate**
  - **[ Traits ]** implement
    - **memory** deallocation
      - `Deref` to behave like a **reference**
      - `Drop` to run **cleanup code when dropped**
        - `drop()` called on closure/lifetime end
        - `std::mem::drop`  forces drop!
          - **WARNING** at risk of **double free**

## **[ LIFETIME ]**

### **-- RAII --**

#### | SINGLE OWNER |

#### `Box<T>`

-

  ```rust
    let b = Box::new(5);
    print!("{}", b);
  ```

  1. allocate
      - // **allocates 5** to **heap**
      - // **points b** to that heap **address**
  2. function
      - // **dereferences b** address
      - // get **value** to **print**
  3. deallocate
      - // on closure/block **exit**

### **-- REF COUNT --**

#### | MULTIPLE OWNER |

- Deallocate on **[ reference count ]** == 0
  - because pointer is no longer in use
- Allow multiple owners of the same data
- Owner is **unknown  @compile time**
- **Immutable** References **ONLY**
  - Interior Mutability via @runtime

#### **[ IMMUTABLE ]**

##### `Rc<T>`

-

  ```rust
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
  ```

  1. let **a** : an Rc with count of **1**
  2. let **b** clone(**&a**) increments the count of **a** to **2**
  3. let **c** clone(**&a**) increments the count of **a** to **3**
  4. When count reaches **0**, **a** is eligible to be **dropped**

#### **[ INTERIOR MUT ]**

##### Outer value stays **IMMUTABLE** while ...

- Mutating an **inner mutable type**
- Uses **unsafe code** wrapped in a safe API
- **[ BORROW ]** rules enforced **@runtime**
  - **HARD** panic @runtime
  - if rules are **VIOLATED**

##### [SINGLE]

- `RefCell<T>`

  -

    ```rust
      struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
      }
    ```

  -

    ```rust
      impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut()
            .push(String::from(message));
        }
      }
    ```

    1. **value ref** returned by
        - `borrow()`
          - **Ref**
        - `borrow_mut()`
          - **RefMut**

##### [MULTIPLE]

###### | single thread |

- `RefCell<T>` + `Rc<T>`
  - Allows shared ownership of mutable data
  -

    ```rust
      let value = Rc::new(RefCell::new(5));

      let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

      let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
      let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

      *value.borrow_mut() += 10;
    ```

    1.
        **value** = RefCell { value: 5 }
    2.
        **value** = RefCell { value: 15 }
    3.
        **a**  = Cons(RefCell { value: 15 }, Nil)
    4.
        **b**  = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    5.
        **c**  = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

###### | multi thread |

- `ArcCell<T>` + `Mutex<T>`
  - Thread safe shared ownership of mutable data
  - @audit : add example when done with chapter

#### **[ REF CYCLE ]**

##### [Memory Leak]

-

  ```rust
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    // Uncommenting will overflow the stack
    // println!("a next item = {:?}", a.tail());
  ```

  1. [LEAK] **'a'** tail from => **Nil** => **'b'**
  2. **Rc::strong_count(&a)** == 2
  3. **Rc::strong_count(&b)** == 2
  4. because both **'a'** and **'b'** always **Rc::strong_count** > 0
  5. `println!("a next item = {:?}", a.tail());` // will overflow

##### **[ WEAK ]** ref

- **Fix** Memory Leak
  - Root cause is STRONG **count never reach 0**
  - **Break Cycle** by  
    - using `Weak<T>` for parent node ref
    - `Weak::upgrade()`gets an `Rc<T>` if it still exists
    - child nodes can still use `Rc<T>`

-

  ```rust
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  ```

  1. // Leaf parent = None // w/ upgrade()
  2. // Leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
  3. **leaf** is **RefCell::new(Weak::new())** to parent **branch**
  4. so that when **branch** weak_count to **0**, branch still **exists**
  5. only **Rc::strong_count == 0** is eligible for **dealloc**

## [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html)

- Has more info on implementing own smart pointers
