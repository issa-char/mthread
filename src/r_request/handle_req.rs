use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use std::net;

pub fn h_req(mut stream: TcpStream) {
        let buf_r = BufReader::new(&mut stream);
        let http_req : Vec<_> = buf_r.lines()
            .map(|result| result.unwrap())
            .collect();
        println!("{:#?}", http_req);
}

