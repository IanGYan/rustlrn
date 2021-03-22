/// echosrv is a TCP Server which echo the input back to Terminal.
/// Current version only supoort UTF-8.

/// Use std Lib only.
use std::{
    io::{self, prelude::*},
    error, net, str, thread,
  };

  /// Main function
  fn main() -> Result<(), Box<dyn error::Error>>{
    // Monitoring port 7878
    let listener = net::TcpListener::bind("127.0.0.1:7878")?;
    loop {
      // Block the calling thread until accept new data from tcp stream
      let (stream, _) = listener.accept()?;
      // Spawn a new thread to handle the stream.
      thread::spawn(move || {
        // Call the handler() function to echo client.
        handler(stream).unwrap()
      });
    }
  }

  /// Function handler() will send back the received data to client by line.
  fn handler(mut stream: net::TcpStream) -> Result<(), Box<dyn error::Error>> {
    // Print the connection info
    println!("{} Connected!", stream.peer_addr()?);
    loop {
      // Create a BufReader to store the stream
      let mut reader = io::BufReader::new(&stream);
      // Create a Vector buf to handle the incoming data.
      let mut buf = Vec::<u8>::new();
      // Get a line to buf
      match reader.read_until(b'\n', &mut buf)? {
        0 => {  //  If can not read anything from stream
          // Print connection closed info to the Terminal.
          println!("Connection closed!");
          // Return Ok Result.
          return Ok(())
        },
        n => {  // Encount the EOL(\n)
          // Convert the buf to UTF-8 String
          let line = str::from_utf8(&buf[..n]);
          // Use patther to handle the Error
          match line {
            Ok(line) => { // If the line is a valid UTF-8 String, print it and write back to client.
              print!("{}: {}", stream.peer_addr()?, line);
              stream.write_all(&buf[..n])?
            },
            Err(err) => print!("Error: {}", err), // If the line is a invalid UTF-8 String, print Error info.
          }
        }
      }
    }
  }