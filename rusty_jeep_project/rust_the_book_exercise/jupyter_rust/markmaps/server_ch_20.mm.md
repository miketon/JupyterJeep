---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# Server

## | Key Concepts |

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

##### -- TCP --

- lower-level protocol
  - describes the details
    - of how information gets from one server to another
    - does NOT specify WHAT that information IS

##### -- HTTP --

- builds on top of TCP
  - defines the CONTENTS of the requests and responses

## | Implementation |

### | Library |

#### std

- std::net::TcpListener
  - Listening to the TCP Connection

### | Asset |

#### html

- hello.html
- 404.html

### | Threaded |

#### Single

#### Multi
