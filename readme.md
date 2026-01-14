# XEO Framework
![Crates.io](https://img.shields.io/crates/v/xeo.svg)
![License](https://img.shields.io/crates/l/xeo.svg)
![Downloads](https://img.shields.io/crates/d/xeo.svg)
XEO is a minimalist, high-performance web framework built from scratch in **Rust**. 


## 🛠Getting Started

**Prerequisites**

Ensure you have an `index.html` file in the root directory of your project.

**Basic Usage**

In your `main.rs`, initialize the XEO server:

```rust
use xeo::{server, version};

fn main() {
    println!("{}", version());
    
    // Starts the server on port 8080 serving "index.html"
    server("index.html", 8080);
}
```

Run the Server

```bash
cargo run
```

Then, open your browser and navigate to http://127.0.0.1:8080.


### License
This project is licensed under the MIT License.