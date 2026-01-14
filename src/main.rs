use xeo::{server, version};


fn main() {
    println!("Version: {}", version());
    let a = server("index.html", 8080);
}