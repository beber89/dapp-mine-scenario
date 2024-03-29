use mio::*;
use mio::net::{TcpStream};
use std::thread;

use std::io::{Read, Write};
use std::str;

#[derive(Debug)]
pub enum TcpNetErr {
    ConnectionFailure,
} 

pub struct Node {
  to_port: u32,
  to_ip_address: String,
  recv: Option<fn(&str)->()>,
}

impl Node {
  pub fn new (ip: String, port: u32) -> Self {
    Node {
      to_port: port,
      to_ip_address: ip,
      recv: None
    }
  }

  pub fn register_read_callback(&mut self, f: fn(&str)->()) {
    self.recv = Some(f);
  }

  pub fn connect(&self) -> Result<(), TcpNetErr> {
    const CLIENT: Token = Token(1);
      let bind_location = format!("{}:{}", self.to_ip_address, self.to_port);
      let poll = Poll::new().unwrap();
      match TcpStream::connect(&bind_location.parse().unwrap()) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");
            poll.register(&stream, CLIENT, Ready::readable(),
              PollOpt::edge()).unwrap();
              let msg = b"Hello!";
              stream.write(msg).unwrap();
              let cb = self.recv;
            // let mut thread_stream = stream.try_clone().expect("Can not clone stream");
            thread::spawn(move || {
              let mut events = Events::with_capacity(1024);
              println!("launching thread");
                loop {
                  println!("in loop");
                    poll.poll(&mut events, None).unwrap();
                    for event in events.iter() {
                        match event.token() {
                            CLIENT => {
                              println!("Reading something");
                              let mut data = [0 as u8; 128]; 
                              match stream.read(&mut data) {
                                Ok(_) => {
                                    println!("Reply OK");
                                    let s = match str::from_utf8(&data) {
                                            Ok(v) => v,
                                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                    };
                                    match cb {
                                      Some(f) => f(s),
                                      None => println!("result: {}", s)
                                    };
                                }, 
                                Err(e) => println!("Failed to receive data: {}", e)
                              }
                                // The server just shuts down the socket, let's just exit
                                // from our event loop.
                                return;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                });
                return Ok(());
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            return Err(TcpNetErr::ConnectionFailure);
        }
    }
  }
}