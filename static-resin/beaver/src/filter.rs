use std::net::{IpAddr};

pub enum Context {
    File(FileContext),
    ClientNetwork(RemoteConnectContext),
    ServerNetwork(ListenConnectionsContext),
    CustomContext(Box<dyn CustomContext>)
}

pub trait CustomContext {}

// infer from file object? 
pub struct FileContext {
    pub file_name: String,
    pub path: String,
}

pub struct RemoteConnectContext {
    pub remote_ip_address: IpAddr,
    pub port: u16,
}

// TODO: Flesh out use case for this; do we need this? 
pub struct ListenConnectionsContext {
    ip_address: IpAddr,
}