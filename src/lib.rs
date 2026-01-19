use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;
use httparse;

pub fn version() -> &'static str {
    "Xeo Framework v0.1.5"
}


pub struct Xeo {
    port: u16,
    ip: String,
    route: HashMap<String, String>,
}


fn get_200(buf: &str) -> String {

    format!("HTTP/1.1 200 OK\r\n\r\n{}", buf)
}

fn get_404() -> String {
    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned()
}




impl Xeo {
    pub fn new(port: u16, ip: String) -> Xeo {
        Xeo {
            port,
            ip: ip.to_owned(),
            route: HashMap::new(),
        }
    }
    pub fn route(&mut self, url: String, filename: String) {
        match fs::read_to_string(&filename) {
            Ok(content) => {
                self.route.insert(url, content);
                println!("Route Registerd, file: {})", filename);
            },
            Err(_) => {
                println!("Error: Could not find file {}", filename);
            }
        }
    }

    pub fn server(&self, html_site: String) {
        let ip = format!("{}:{}", self.ip, self.port);
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


        for stream in listener.incoming() {
            match stream {
                Ok(mut k) => {
                    println!("New Connection");

                    let mut buffer = [0; 2048];
                    match k.read(&mut buffer) {
                        Ok(l) => {
                            println!("Read");
                        }
                        Err(_) => continue,
                    }


                    let mut headers = [httparse::EMPTY_HEADER; 16];

                    let mut req = httparse::Request::new(&mut headers);

                    let res = match req.parse(&buffer) {
                        Ok(res) => res,
                        Err(_) => {
                            println!("Error parsing");
                            continue;
                        }
                    };

                    if let Some(path) = req.path {
                        println!("Path: {}", path);
                        let resp = match self.route.get(path) {
                            Some(cont) => get_200(cont),
                            None => get_404(),
                        };
                        k.write_all(resp.as_bytes()).expect("Error connecting");


                    }

                },
                Err(e) => {
                    println!("Error: {}", e);


                }
            }
        }
    }
}
