#![deny(warnings)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, self};

#[derive(Debug)]
struct Request {
    pub method: String,
    pub uri: String,
    pub body: Option<String>,
}

fn parse_request_line(request: &str) -> Result<Request, String> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;

    if method != "GET" {
        return Err("Unsupported Method".to_string());
    }

    let uri = parts.next().ok_or("URI not specified")?;

    Ok(Request {
        method: method.to_string(),
        uri: uri.to_string(),
        body: None,
    })
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = vec![];

    stream.read_to_end(&mut buffer)?;

    let request = String::from_utf8(buffer).unwrap();
    let request_line = request.lines().next().unwrap();

    match parse_request_line(&request_line) {
        Ok(mut request) => {
            request.body = Some("Hello World!".to_string());
            println!("\n Request: ");
            println!("\n Method: {}", request.method);
            println!("\n URI: {}", request.uri);
            if request.body != None {
                println!("\n Body: {}", request.body.unwrap());
            }
        },
        Err(e) => println!("{:#?}", e),
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let ip = "127.0.0.1:8080";
    let listener = TcpListener::bind(ip)?;

    println!("Starting server at {}", ip);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                listener.accept()?;
                println!("Accepted Stream");
                handle_client(stream)?;
            },
            Err(e) => {
                println!("{}", e);
            },
        }
    }

     Ok(())
}

#[cfg(test)]
mod tests {

}

