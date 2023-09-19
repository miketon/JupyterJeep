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
5. Improve the throughput of our server with a thread pool

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

##### 🪡 ==[ TCP ]== 🪡

- **TRANSPORT** layer
  - lower-level protocol
  - describes the details
    - of HOW information gets from one server to another
    - does NOT specify WHAT that information IS

##### 💬 ==[ HTTP ]== 💬

- **CONTENT** layer
  - builds on top of TCP
  - hyper-text
    - defines the CONTENTS of requests and responses

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
              - 'loopback address' = 127.0.0.1
                - this particular address is the SAME on every computer
                - by convention is a MIRROR used for TESTING
          - Port Number
            - `7878`
              - 'rust' typed on a telephone
              - unlikely to conflict with any other web server
                - HTTP isn't normally accepted on this port
      - returns `Result<T,E>`
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

### | url | 

#### -- nav bar --

- `http://127.0.0.1:7878/`
- `http://127.0.0.1:7878/sleep`

##### 💬 HTTP **Request**

- ```sh
    Method Request-URI HTTP-Version CRLF
    headers CRLF
    message-body
  ```

  - `Request: [ ... ]`
    - Method Request-URI HTTP-Version CRLF

      - ```sh
          "GET / HTTP/1.1",
        ```

        - | Method |
          - holds information about what the client is requesting
          - `GET`
            - asking for information
          - 'POST'
            - @audit ?
        - | Request-URI |
          - `/`
            - Uniform Resource Identifier
              - @audit : How/why is this different from URL
        - | HTTP-Version |
          - `HTTP/1.1`
            - HTTP version the client uses
              - @audit : explain each version
                - HTTP/1.0
                - HTTP/1.1
                - HTTP/2.0
                - HTTP/3.0
        - | CRLF |
          - `\r\n`
            - carriage return and line feed
            - ends the REQUEST line
    - headers CRLF

      - ```sh
          "Host: 127.0.0.1:7878",
        ```

        - | headers |
          - domain name
            - 127.0.0.1:
              - localhost ip address
          - port number
            - 7878
              - 'rust' typed out on a phone
          - | config |
            - User-Agent
              - contains info on client making the request
              - "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0"
            - Accept
              - media
                - specifies the media types the client can process
                  - and in what priority order, helping the server
                  provide a response in a suitable format
                - "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"

              - language
                - indicates the client's language preferences
                - "Accept-Language: en-US,en;q=0.5"
              - encoding
                - specifies the content-coding values that the client supports for the response
                - "Accept-Encoding: gzip, deflate, br"
            - DNT
              - requests that the user's behavior not be tracked
              - "DNT: 1"
                - Do Not Track bool
            - Connection
              - "Connection: keep-alive"
                - controls whether the network connection stays
                open after the current transaction finishes
                  - @audit : Explain this
            - Upgrade-Insecure_Requets
              - "Upgrade-Insecure-Requests: 1"
                - tells the server that the client wants to
                upgrade the current connection to HTTPS
                  - @audit : Explain this
            - Sec-Fetch
              - Dest
              - Mode
              - Site
              - User
            - Cache-Control
        - | CLRF |

    - message-body

      - ```sh
          -- GET typically DOES NOT have a body --
        ```

        - @audit : POST body example?

### | action |

#### GET

#### POST
