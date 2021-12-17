# BEAVER

BEAVER is a Resin-style IFC Library for Rust built by Sreshtaa Rajesh and Livia Zhu for CSCI2390: Privacy-Conscious Computer Systems. In this repo, we have 3 folders: `beaver`, `beaver-derive`, and `example`. `beaver` and `beaver-derive` comprise the BEAVER library, with `beaver-derive` providing our custom derive macro. `example` houses a simple Grade demo to show how programmers can use BEAVER. 

**To run the example, please navigate to the `static-resin` directory and run `cargo run`!**

Currently, we are unable to provide a demonstration of sending policied data across network sockets, due to the fact that we don't have a permanently listening socket. 

If you would like to try out sending policied data over network connections, feel free to fork the repository and modify the IP addresses/ports in the `main.rs` file inside the `example` folder to IP addresses of devices that you have access to and can set up a listening socket from. Once you have this, run `nc -l <port-number>` to open a listening socket in another terminal at a given port. You may also need to update the filter contexts at the end of the file. 

Currently, if any of the sockets are not listening, the thread panics since the TCP connection failed. Then, run `cargo run`. You may find that the while the program is trying to connect to a socket, the code looks like it's hanging, but it should either fail by itself, or you can press ^C to abort the connection. 

Let's build some dams! 
