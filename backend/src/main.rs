use std::{net::TcpListener, fs::File, io::Read};
use tinyhttp::prelude::*;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello WASM!"
}

#[get("/wildcard/:")]
fn wildcard(req: Request) -> String {
    if let Some(w) = req.get_wildcard() {
        w.clone()
    } else {
        String::from("no wildcard :(")
    }
}

fn main() {
    let sock = TcpListener::bind("127.0.0.1:8080").unwrap();
    let routes = Routes::new(vec![hello(), wildcard()]);
    let config = Config::new().mount_point("./public/").routes(routes);
    let http = HttpListener::new(sock, config);
    http.start();
}
