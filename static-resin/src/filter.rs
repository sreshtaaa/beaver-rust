use std::io;
use std::net;

pub enum Context {
    File(FileContext),
    ClientNetwork(ClientNetworkContext),
    ServerNetwork(ServerNetworkContext),
}

pub struct FileContext {
    pub(crate) file_name: String,
    path: String,
}

pub struct ClientNetworkContext {
    ip_address: std::net::IpAddr,
}

pub struct ServerNetworkContext {
    ip_address: std::net::IpAddr,
}