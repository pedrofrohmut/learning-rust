use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, time::Duration, thread};

use server::ThreadPool;

fn main()
{
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool = ThreadPool::new(4);

    // Take(2) to test shutting down the server
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" =>
            ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(2));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ =>
            ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\nContent-Length: {length}\n\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
