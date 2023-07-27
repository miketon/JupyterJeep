---
markmap:
   colorFreezeLevel: 2
   maxWidth: 400
---

# Smart Pointers

## **[ HEAP ]**

### **[ POINTER ]**

#### **concept**

- variable containing an **address** in memory
- this address **points** at some **data**
- **deref** a pointer follows to get **value**
- smart pointer OWN data
  - references only BORROW

#### **usage**

- **allocate**
  - fixed **size**
  - enables **[Indirection]** (recursion)
- update **address**
  - **FLASH** transfer **[Ownership]**
  - Does **NOT** copy LARGE amounts of Data
- **deallocate**
  - **[ Traits ]** implement
    - **memory** deallocation
      - `Deref` to behave like a **reference**
      - `Drop` to run **cleanup code when dropped**

## **[ OWNERSHIP ]**

### **[ SINGLE ]**

#### [ lifetime ] **RAII**

##### `Box<T>`

-

  ```rust
    let b = Box::new(5);
    print!("{}", b);
  ```

  1. allocate
      - // **allocates** `5` to **heap**
      - // **points** `b` to that heap **address**
  2. function
      - // **dereferences** `b` address
      - // get `value` and print
  3. deallocate
      - // on closure/block exit

### **[ MULTI ]**

#### [ lifetime ] **REF COUNT**

- Deallocate on **[ reference count ]** == 0
- Allow multiple owners of the same data
- **Immutable** References **ONLY**
  - Interior Mutability via @runtime

#### `Rc<T>`

- `let a = Rc::new(5);` creates an Rc with a count of 1
- `Rc::clone(&a)` increments the count  
- When count reaches 0, the value is dropped
- Implement Drop to decrement the count when dropped

## **[ LIFETIME ] RAII**

### Reference Count

#### **[ COMPILE ]**

#### **[ RUNTIME ]**

## `RefCell<T>`

- Enforce the borrowing rules at runtime instead of compile-time
- Allow mutating an inner value while the outer value is immutable  
- Use Ref and RefMut smart pointers to get references
- Panic at runtime if rules are violated

## `Interior Mutability`

- Pattern of mutating an immutable type by mutating an inner mutable type  
- Uses unsafe code wrapped in a safe API

## Combining `Rc<T>` and `RefCell<T>`

- Allows shared ownership of mutable data
- `RefCell<T>` enforces borrowing rules at runtime

## Reference Cycles

- Can cause memory leaks if counts never reach 0
- Break cycles using `Weak<T>` instead of `Rc<T>` for one of the references
- `Weak::upgrade()`gets an `Rc<T>` if it still exists  

## The Rustonomicon

- Has more info on implementing own smart pointers
