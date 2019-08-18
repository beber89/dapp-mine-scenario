mod node;
use node::Node;
use std::io::{stdin,Write};

//TODO: 
// [ ] Create event loop for client implementing readable and writeable
// [x] Add streams to Node in which it will be listened to by main()
//  > Used a callback instead

fn main() {
    let mut node = Node::new("127.0.0.1".to_owned(), 8080);
    node.registerReadCallback(|s| println!("handling data from main: {}", s) );
    node.connect().expect("ERROR!!");

    println!("Sent Hello, awaiting reply...");
    

    let mut s=String::new();
    print!("Please press enter to exit ");
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    println!("end program");
}
