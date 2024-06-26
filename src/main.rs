use std::net::TcpListener;
mod r_request;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("connection established");
        r_request::handle_req::h_req(stream);
    }

    

}
