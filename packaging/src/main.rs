
/// This module contains network-related functionality.
pub mod network {
    /// Establishes a connection to the network.
    fn connect() {
        println!("The network connected");
    }

    /// Initializes the network by first connecting and then performing additional setup.
    pub fn initialize() {
        connect();
        println!("The network initialized");
    }
}

fn main() {

    let network = network::initialize();
    println!("network {:?}", network);
}
