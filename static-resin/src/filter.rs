use std::io;
use std::net;

mod filter {
    pub enum Context {
        File(FileContext),
        ClientNetwork(ClientNetworkContext),
        ServerNetwork(ServerNetworkContext),
    }
    
    pub struct FileContext {
        file_name: String,
        path: String,
    }
    
    pub struct ClientNetworkContext {
        ip_address: std::net::IPAddr,
    }
    
    pub struct ServerNetworkContext {
        ip_address: std::net::IPAddr,
    }
}