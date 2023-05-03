// https://docs.rs/ssh2/latest/ssh2/
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = &args[1];
    let username = &args[2];
    let password = &args[3];

    // Connect to the local SSH server
    let tcp = TcpStream::connect(address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&username, &password).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());

}
