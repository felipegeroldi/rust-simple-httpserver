use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use crate::http::request::ParseError;
use std::convert::TryFrom;
use crate::http::Response;
use crate::http::StatusCode;

pub struct Server {
    address: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            address: addr
        }
    }

    pub fn run(&self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);
    
        let listener = TcpListener::bind(&self.address)
            .unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer: [u8;1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                }
                Err(_) => continue
            }
        }
    }
}