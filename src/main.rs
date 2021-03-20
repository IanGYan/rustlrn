/*
Echo Server
v0.1.0
A TCP Server which can echo the connection.
Usage: echosrv

Author: Ian Yan @ian@adhub.cn
Date: 2021-Mar-20
*/

use std::{
    io::{self, prelude::*},
    error, net, str, thread,
  };

  /// Main function
  fn main() -> Result<(), Box<dyn error::Error>>{   // Throw exceptions
    let listener = net::TcpListener::bind("127.0.0.1:7878")?;   // Monitoring port 7878
    loop {
      let (stream, _) = listener.accept()?; // Block the calling thread until accept new data from tcp stream
      thread::spawn(move || {   //Create a thread to handle the echo
        handler(stream).unwrap();   // Handle the echo
      });
    }
  }

  /// Function echo will send back the received data to client
  fn handler(mut stream: net::TcpStream) -> Result<(), Box<dyn error::Error>> {
    println!("Receive data from {}", stream.peer_addr()?);  // Print the source addr
    loop {
      let mut reader = io::BufReader::new(&stream); // Store the stream to BufReader
      let mut buf = vec![]; // Create a buffer to format the received data
      match reader.read_until(b'\n', &mut buf)? {   // from BufReader store data to buffer until '\n' or EOF
        0 => {  //  EOF
          println!("Connection closed");    // Print Closed info to terminal
          return Ok(()) // Return Ok Result
        },
        n => {  // End of Line
          print!("{}", str::from_utf8(&buf[..n])?); // Convert buffer to String and print
          stream.write_all(&buf[..n])?; // Write back the data to Client
        }
      }
    }
  }