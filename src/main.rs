use std::io::{self, Write, BufReader, BufRead};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};

fn main() {
    let bind_ip = Ipv4Addr::new(127, 0, 0, 1);
    let bind_port = 1234;

    let cs = SocketAddrV4::new(bind_ip, bind_port);

    let listn = TcpListener::bind(cs);
    let listener = match listn {
        Ok(l) => l,
        Err(e) => {
            print!("{}", e);
            exit(0);
        }
    };

    let (mut client_socket, client_address) = listener.accept().unwrap();
    println!("from: {}", client_address);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("String expected");
        if input == "quit" {
            break;
        }
        input.push('\0');

        client_socket.write(&mut input.as_bytes());

        let mut buffer: Vec<u8> = Vec::new();

        let mut reader = BufReader::new(&client_socket);
        reader.read_until(b'\0', &mut buffer);

        println!("Received: {}", String::from_utf8_lossy(&buffer).trim_end());
    }

    drop(listener);
}
