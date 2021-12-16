# BEAVER

BEAVER is a Resin-style IFC Library for Rust built by Sreshtaa Rajesh and Livia Zhu for CSCI2390: Privacy-Conscious Computer Systems. In this repo, we have 3 folders: `beaver`, `beaver-derive`, and `example`. `beaver` and `beaver-derive` comprise the BEAVER library, with `beaver-derive` providing our custom derive macro. `example` houses a simple Grade demo to show how programmers can use BEAVER. 

To run the example, please navigate to the `static-resin` directory. Run `nc -l 5000` to open a listening socket in another terminal. Currently, if any of the sockets are not listening, the thread panics since the TCP connection failed. Then, run `cargo run`. 
