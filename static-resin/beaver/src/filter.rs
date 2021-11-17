use std::net::{IpAddr};

pub enum Context {
    File(FileContext),
    ClientNetwork(RemoteConnectContext),
    ServerNetwork(ListenConnectionsContext),
}

// infer from file object? 
pub struct FileContext {
    pub file_name: String,
    pub path: String,
}

pub struct RemoteConnectContext {
    pub remote_ip_address: IpAddr,
    pub port: u16,
}

pub struct ListenConnectionsContext {
    ip_address: IpAddr,
}