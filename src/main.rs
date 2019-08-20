extern crate serde;

use std::io::{stdin};
mod node;
mod message;
use node::Node;
mod tests;
#[macro_use]
extern crate serde_derive;

//TODO: 
// [ ] Modify server to broadcast messages to other streams from table when recv nonce from one
//   [ ] This node loop stops when server get request from other client
// [x] Create a message object with enum type
// [ ] run several clients (Nodes) with different IDs
//   [ ] add command line arguments to assign ID from cargo run
// [ ] Create event loop for client implementing readable and writeable
// [x] Add streams to Node in which it will be listened to by main()
//  > Used a callback instead

fn main() {
    let mut node = Node::new("127.0.0.1".to_owned(), 8080);
    node.register_read_callback(|s| println!("handling data from main: {}", s) );
    node.connect().expect("ERROR!!");

    println!("Sent Hello, awaiting reply...");
    

    let mut s=String::new();
    print!("Please press enter to exit ");
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    println!("end program");
}