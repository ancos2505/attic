use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::stats::Stats;
use crate::terminal::Terminal;
use crate::AppResult;

pub struct HttpServer;
impl HttpServer {
    pub fn run() -> AppResult<()> {
        let port = 8080;
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

        println!("Listening for connections on port {port}");
        let prev_stats: Arc<Mutex<BTreeMap<String, (u64, u64)>>> =
            Arc::new(Mutex::new(BTreeMap::new()));

        {
            let stats_mutex = Arc::clone(&prev_stats);
            let res = stats_mutex.lock();
            if let Ok(mut data) = res {
                Stats::new().render(&mut data)?;
            }
        }

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stats_mutex = Arc::clone(&prev_stats);
                    thread::spawn(move || {
                        if let Ok(mut data) = stats_mutex.lock() {
                            let _ = Self::handle_client(&mut data, stream);
                        }
                    });
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
        Ok(())
    }
    fn handle_client(
        prev_stats: &mut BTreeMap<String, (u64, u64)>,
        stream: TcpStream,
    ) -> AppResult<()> {
        Self::handle_read(&stream);

        Self::handle_write(prev_stats, stream)?;
        Ok(())
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
    fn handle_write(
        prev_stats: &mut BTreeMap<String, (u64, u64)>,
        mut stream: TcpStream,
    ) -> AppResult<()> {
        // let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        let http_response_header =
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
        let html_page_content = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="refresh" content="3">
    <title>Kindle Monitor</title>
</head>
<body>
<canvas id="myCanvas" width="519" height="593" style="border:1px solid #000000;">
        Your browser does not support the HTML5 canvas tag.
    </canvas>
    <script>
        const START_WIDTH = 25;
        const START_HEIGHT = -35;
        const FONT_SIZE = 25;
        const FONT_WEIGHT = 15;
        const MAX_STR_SIZE = 36;
document.addEventListener("DOMContentLoaded", function () {
    function line(line) {
        return START_WIDTH + (FONT_SIZE * (line - 1))
    }
    function column(col) {
        return START_HEIGHT - (FONT_WEIGHT * (MAX_STR_SIZE - col))
    }
    function printChar(c, col, lin) {
        ctx.fillText(c, column(col), line(lin));
    }
    function printString(str, col, lin) {
        for (var i = 0; i < str.length; i++) {
            var c = str[i];
            printChar(c, col + i, lin)
        }

    }

    const canvas = document.getElementById('myCanvas');
    const ctx = canvas.getContext('2d');

    const canvasFontStyle = '' + FONT_SIZE + 'px Monospace';
    // Clear the canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.rotate(3 * Math.PI / 2);

    ctx.textAlign = "right";
    ctx.fillStyle = '#000';
    ctx.font = canvasFontStyle;

    /// BEGIN
    // Print rulers
    // {
    //     printString('000000000111111111122222222223333333', 3, 1);
    //     printString('123456789012345678901234567890123456', 3, 2);
    //     for (var i = 1; i < 20; i++) {
    //         if (i < 10) {
    //             printChar('0', 1, 2 + i);
    //         }
    //         printChar(i, 2, 2 + i);
    //     }
    // }
    // printChar('A', 5 + 2, 5 + 2);
    

"#;
        let http_response_footer = "\n});\n</script></body></html>\r\n";

        // let terminal_output = Terminal::new().render()
        // .replace("\n", "<br/>")
        // ;
        // println!("");
        // dbg!(&terminal_output);
        let stats = Stats::new().render(prev_stats)?;
        let mut response = "".to_string();
        response.push_str(http_response_header);
        response.push_str(html_page_content);
        response.push_str(&stats);
        // response.push_str(&terminal_output);
        response.push_str(&http_response_footer);

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}
