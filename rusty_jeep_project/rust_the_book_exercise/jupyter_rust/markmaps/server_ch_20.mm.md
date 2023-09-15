---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# Network

## 🔑 | Key Concepts | 🔑

1. Learn a bit about TCP and HTTP
2. Listen for TCP connections on a socket
3. Parse a small number of HTTP requests
4. Create a proper HTTP response
5. Imporve the throughput of our server with a thread pool

### | Protocols |

- protocols DEFINE the contents of REQUESTS and RESPONSE

#### request-response

- client initiates requests
- server listens to the requests
  - provides a response to the client
    - HTTP sends data over TCP
      - technically possible to use HTTP with other protocols
        - but VAST majority of cases HTTP sends data over TCP
    - We'll work with the raw bytes of TCP and HTTP requests and responses

##### 👁️‍🗨️ ==[ TCP ]== 👁️‍🗨️

- lower-level protocol
  - describes the details
    - of how information gets from one server to another
    - does NOT specify WHAT that information IS

##### 💬 ==[ HTTP ]== 💬

- builds on top of TCP
  - defines the CONTENTS of the requests and responses

## -- SERVER --

### | Environment |

#### | Library |

##### std

- 🛜 std::net::TcpListener 🛜
  - Listening to the TCP Connection
  - `TcpListener::bind("127.0.0.1:7878")`
    - `::bind()`
      - 'binding to a port'
        - connecting to a `port` to `listen`
      - 127.0.0.1:7878
        - `:`
          - Ip Address
            - `127.0.0.1`
              - represents your computer
              - this value specifically is the 'loopback address'
                - this particular address is the SAME on every computer
                - and is a mirror that is conventionally used for testing
          - Port Number
            - `7878`
              - 'rust' typed on a telephone
              - unlikely to conflict with any other web server
                - HTTP isn't normally accepted on this port
      - `returns` Result<T,E>
        - `T`
          - `socket` server
            - 🎧 ==[ TcpListener ]== 🎧
              - used to `listen` for incoming connections from client
              - ⛓️ ==[ TcpStream ]== ⛓️
                - used to `read` from and `write` to a `connection`
                  - `struct` representing a `TCP stream` between
                    - `local` socket
                    - `remote` socket
                - | PROCESS | each `TcpStream`
                  - 'open connection' between client and server
                  - 'connection' is name for the full `request and response` process
                    1. a client `connects` to the server
                    2. the server `generates` a response
                    3. the server `closes` the connection
                  - `read` from the `TcpStream` to see :
                    1. `read` what the `client sent`
                    2. `write` our response to the `TcpStream`
                    3. to send data back to the client
                - | ERROR | potential of each `TcpStream`
                  - we may be iterating over connections ATTEMPTS
                    - as opposed to iterating ACTUAL connections
        - `E`
          - `Error` indicates that it's possible for `binding()` to `FAIL`
          - | EXAMPLES |
            - `administrator` privileges
              - nonadministrators can only listen on `ports > 1023`
                - connecting to port `80` is for admins ONLY
            - port `collision`
              - two instances of our programs listening to the SAME port

#### | Asset |

##### html

- hello.html ✅
- 404.html ❌

### | Implementation |

#### | Import |

- ```rust
    use std::{
      fs,
      io::{prelude::*, BufReader},
      net::{TcpListener, TcpStream},
    }
  ```

  - `fs`
    - filesystem module to work with files and directories
      - read from files
      - write to files
  - `io::`
    - `prelude::*`
      - brings in IO prelude to include traits like :
        - 'prelude' is a common term in Rust for a small collection of items
        - usually Traits
      - `Read`
      - `Write`
    - `BufReader`
      - Buffering speeds up your I/O operations because :  
        - it reduces the number of read system calls
        - by reading large chunks at once
        - struct provided by Rust
  - `net::`
    - 🎧 `TcpListener`
      - struct that represents a TCP `socket server`
      - used to `listen for` incoming connections from clients
    - ⛓️ `TcpStream`
      - struct representing a TCP `stream between a local and a remote socket`
      - used to r`ead from and write to` the connection

#### | Listen |

- ```rust
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  ```

  - New TCP Listener is being created
    - 'bind()' to
      - ip address : 127.0.0.1
      - port : 7878
    - 'unwrap()' will panic and halt program on ERROR
      - such as address/port COLLISION

#### | Response |

- ```rust
    for stream in listener.incoming() {
      let stream = stream.unwrap();
      handle_connection(stream);
    }
  ```

  - `listener.incoming()`
    - loop through every connection made to listener
      - new TcpStream created
        - Each time a client makes a connection to the server
    - `returns` an `iterator` over these `TcpStreams`

  - `stream.unwrap()`
    - 'unwrap' will panic and halt program on ERROR

  - `handle_connection(stream)`
    - handle the connection
      - `read` the stream
      - `process` the request
      - `send` a response

##### | Threaded |

- ```rust
    fn handle_connection(mut stream: TcpStream){
      // ...
      let response = format!("{handle response HERE}");
      // ...
      stream.write_all(response.as_bytes()).unwrap();
    }
  ```

  - handle connection stream
    - read
    - process
    - send

###### Single

###### Multi

## -- CLIENT --

### | url nav |

### | action |

#### GET

#### POST
