use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

pub fn version() -> &'static str {
    "Xeo Framework v0.1.2"
}

fn get_200(buf: &str) -> String {
    format!("HTTP/1.1 200 OK\r\n\r\n{}", buf)
}
fn get404() -> String {
    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned()
}


pub fn server(html_site: &str, port: u16) {
    let ip = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&ip) {
        Ok(l) => {
            println!("Server Started");
            l
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let main_page = match fs::read_to_string(html_site) {
        Ok(content) => {
            println!("Content Acquired");
            content
        },
        Err(e) => {
            println!("Error loading content");
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut k) => {
                println!("New Connection: {}", &ip);
                let response = get_200(&main_page);
                k.write_all(response.as_bytes()).expect("Error connecting");

            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }




}
