use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::terminal::Terminal;
use crate::AppResult;

pub struct HttpServer;
impl HttpServer {
    pub fn run() -> AppResult<()> {
        let port = 8080;
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
        println!("Listening for connections on port {port}");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| Self::handle_client(stream));
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
        Ok(())
    }
    fn handle_client(stream: TcpStream) {
        Self::handle_read(&stream);

        Self::handle_write(stream);
    }
    fn handle_read(mut stream: &TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("{}", req_str);
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }
    fn handle_write(mut stream: TcpStream) {
        // let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        let http_response_header =
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
        let html_page_content = r#"<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="refresh" content="3">
  <title>Kindle Linux Terminal Emulator</title>
  <style>
    body {
      margin: 0;
      padding: 0;
      background-color: white;
      color: black;
      font-family: monospace;
      font-size: 10px;
      height: 100vh;
      width: 100vw;
      /* display: flex; */
      /* justify-content: flex-start; */
      /* align-items: flex-end; */
    }

    #terminal {
      font-size: 48px;
      /* border: 2px solid black; */
      writing-mode: tb-rl;
      -webkit-transform: rotate(270deg);
      -moz-transform: rotate(270deg);
      -o-transform: rotate(270deg);
      -ms-transform: rotate(270deg);
      transform: rotate(270deg);
      transform-origin: bottom left;
      width: 1220px;
      height: 1080px;
      position: absolute;
      /* margin-top: 30px; */
      /* margin-bottom: 0px; */
      /* margin-left: 0px; */
      bottom: 110px;
      left: -115px;
    }
  </style>
</head>

<body>
  <div id="terminal">
    <p>
"#;
        let http_response_footer = "</p></div></body></html>\r\n";

        let terminal_output = Terminal::new().render()
        // .replace("\n", "<br/>")
        ;
        println!("");
        dbg!(&terminal_output);

        let mut response = "".to_string();
        response.push_str(http_response_header);
        response.push_str(html_page_content);
        response.push_str(&terminal_output);
        response.push_str(&http_response_footer);

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }
}
