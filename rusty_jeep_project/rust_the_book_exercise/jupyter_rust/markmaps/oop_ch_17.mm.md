---
markmap:
   colorFreezeLevel: 2
   maxWidth: 500
---

# **| OOP |**

## Features

### | Objects |

- Conventional
  - **`Class`**
    - contain **both**
      - **Data**
      - **Behavior**
    - extend
      - can **add** Data
      - can **add** Behaviour
- | RUST | **`impl`**
  - [ emulate ]
    - **State as Pattern**
      - DATA
        - **`struct`**
        - **`enum`**
        - | RUST | can **NOT** add Data
      - BEHAVIOUR
        - **`trait`** impl
          - trait **bound**
          - trait **object**
        - | RUST | can add **trait** (behaviour)
  - [ encode ]
    - **State as Type**
      - ! invalid state **UNREPRESENTABLE** !
      - **@compile** time check
      - | type **swap via shadow** |
        - **Post**::init > **Draft**::req_rev > **PendingReview**::approved > **Post**

### | Encapsulation |

- **public** interface
  - | src/mod.rs |
    - `struct` **AverageCollection**
      - **public**
        - `pub`
          - 'AverageCollection' **struct**
      - **private**
        - |default|
          - ::**list**
          - ::**average**

    -

      ```rust
        // pub allows Object to be accessed outside of module
        pub struct AveragedCollection {
          list: Vec<i32>, // default private
          average: f64,   // default private
        }
      ```

- **private** impl
  - | src/mod.rs |
    - `impl` **AverageCollection**
      - ::**[methods]**
        - **public**
          - `pub`
            - ::**add(i32)**
            - ::**remove()**
            - ::**average()**
        - **private**
          - |default|
            - ::**update_average()**
    -

      ```rust
        impl AveragedCollection {
          pub fn add(&mut self, value: i32){
            self.list.push(value);
            self.update_average();
          }

          pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
              Some(value) => {
                self.update_average();
                Some(value)
              }
              None => None,
            }
          }

          pub fn average(&mut self) -> f64 {
            self.average
          }

          // internally updates the average, so has no pub modifier, default private
          fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
          }
        }
      ```

### | Inheritance |

- conventional
  - expresses type system
  - code sharing compression

- | RUST |
  - trait bounds
  - trait objects

## Usage

### **patterns**

#### | OOP |

- oop.h
- oop.c

#### | RUST |

##### Library : **Gui**

###### Trait Objects

- | DATA |
  - | src/mod.rs |

    -

      ```rust
        pub trait Draw {
          fn draw(&self);
        }
      ```

      1. // 'objects'
          - `struct`
          - `enum`
            - trait could also be `impl` for **enum**
              - not just **struct**
                - | src/mod.rs |

                  -

                    ```rust
                      enum Animal {
                        Dog,
                        Cat,
                      }
                    ```

                  -

                    ```rust
                      impl Sound for Animal {
                        fn sound(&self) -> &'static str {
                          match self {
                            Animal::Dog => "woof",
                            Animal::Cat => "meow",
                          }
                        }
                      }
                    ```
  
- | METHOD |
  - `trait`

    - **bound**
      - | homogenous |
        - **-**
          - **1:1** 
            - generic`<T>` => concrete`<T>`
        - **+**
          - **fully compiled**
            - inlined to concrete impl (**monomophization**)
            - benefits from compiler **optimization**
      - | src/mod.rs |

        -

          ```rust
            pub struct Screen<T: Draw> {
              pub components: Vec<T>
            }
          ```

          1. @audit : describe `Vec<T>`

        -

          ```rust
            impl<T> Screen<T>
              where T: Draw,
            {
              pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
              }
            }
          ```

    - **object**
      - | heterogenous |
        - **+**
          - **1:N**
            - generic => **1..N** 
            - `impl` trait
              - **@runtime** method **lookup table**
        - **-**
      - | src/mod.rs |

        -

          ```rust
            pub struct Screen {
                pub components: Vec<Box<dyn Draw>>
            }
          ```

          1. `dyn` Draw - **dynamically** dispatch each `impl` **Draw trait**
              - **@compile guard**
                - trait **method impl**
                - @runtime trait **method call**
              - | **Types w/Draw** |
                - ONLY concerned with **messages the value responds to**
                  - as opposed to the **value's concrete type**
                - | FUTURE proofed | **Types*
                  - Box<**Button**>

                    -  

                      ```rust
                        pub struct Button {
                          pub width : u32,
                          pub height : u32,
                          pub label : String,
                        }
                      ```

                    -

                      ```rust
                        impl Draw for Button {
                          fn draw(&self) {
                            // ... draw button here
                          }
                        }
                      ```

                    - Box<**SelectBox**> ...
                    - Box<**TextField**> ...
                    - Box<**Slider**> ...
          2. using Trait method lookup --| **table** |-- **@runtime**

        -

          ```rust
            impl Screen {
                pub fn run(&self) {
                    for components in self.components.iter() {
                        component.draw();
                    }
                }
            }
          ```

          1. **Screen**::`draws()` heterogeneous Components

      - -- orchestrate --
        - | src/main.rs |

          -

            ```rust
              fn main() {
                let screen = Screen {
                  vec![
                    Box::new(SelectBox{ ... }),
                    Box::new(Button{ ... }),
                  ]
                };

                screen.run();
              }
            ```

            1. `Box<dyn Draw>` == **Duck Typing** Screen Components
                - **Screen** just calls the **Component::draw()**
                  - NO need to CHECK Type if Button, TextField ...etc
                  - Just that it `impl` the **Draw trait methods**

      - -- trade offs --
        - **dynamic dispatch**
          - incurs **small @runtime cost**
            - when compared to static dispatch
        - **skips optimization potential** available to static compile

##### Library : **Blog**

###### | API |

- **Unit Test**

  -

    ```rust

      {

        // returns Draft with empty string ""
        let mut post = blog::Post::new();

        // new Post is a Draft and should be an empty string
        post.add_text("I battled a bee this week");
        assert_eq!("", post.content());
    
        // Draft => PendingReview, post should still be empty
        post.request_review();
        assert_eq!("", post.content());

        // PendingReview => Published, post content now 
        // publically available to read
        post.approve();
        assert_eq!("I battled a bee this week", post.content());

      }

    ```

###### State as Pattern

- Trade-offs and advantages of the state pattern in Rust

- | TRAIT |
  - **State**

    -

      ```rust
        trait State {
          fn request_review(self: Box<Self>) -> Box<dyn State>;
          fn approve(self: Box<Self>) -> Box<dyn State>;
          fn content<'a>(&self, post: &'a Post) -> &a' str {
            ""
          }
        }
      ```

      1. **State::content()** has a default `impl`
          - To support **Post::Draft** and **Post::PendingReview**
          - **Post::Published** needs to `override` empty **Post::content**

- | STRUCT |
  - **Post**

    -

      ```rust
        pub struct Post {
          state: Option<Box <dyn State>>,
          content: String,
        }
      ```

      1. `impl` Post

          -

            ```rust
              impl Post {
                pub fn new() -> Post {
                  state: Some(Box::new(Draft {})),
                  content: String::new(),
                }

                pub fn add_text(&mut self, text: &str) {
                  self.content.push_str(text);
                }

                // Post::methods to control State::methods
                // - Post is delegating to State
                pub fn request_review(&mut self) {
                  if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                  }
                }

                pub fn approve(&mut self) {
                  if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                  }
                }

                pub fn content(&mut self) {
                  self.state.as_ref().unwrap().content(self)
                }
              }

            ```

  - **Draft**

    -

      ```rust
        struct Draft {}
      ```

      1. `impl` State for Draft

  - **PendingReview**

    -

      ```rust
        struct PendingReview {}
      ```

      1. `impl` State for PendingReview

  - **Published**

    -

      ```rust
        struct Published {}
      ```

      1. `impl` State for Published

###### State as Type

- Encoding states and behaviors as types in Rust

## Summary