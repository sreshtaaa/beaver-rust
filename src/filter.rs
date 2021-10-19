use std::io;
use std::net;

enum Context {
    file(FileContext),
    clientNetwork(ClientNetworkContext),
    serverNetwork(ServerNetworkContext),
}

struct FileContext {
    file_name: String,
    path: String,
}

struct ClientNetworkContext {
    ip_address: net::IPAddr,
}

struct ServerNetworkContext {
    ip_address: net::IPAddr,
}