use std::net::{ TcpStream, TcpListener };
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;
use thread_pool::ThreadPool;


fn handle_connection(mut stream:TcpStream, wait:u64){
  let mut buf = [0;512];
  let request = stream.read(&mut buf).unwrap();
  // thread::sleep(Duration::from_secs(wait));
  if buf.starts_with(b"HTTP/1.1 200 OK \r\n\r\n"){
    let content = fs::read_to_string("./views/index.html").unwrap();
    stream.write(format!("HTTP/1.1 200 OK{}", content).as_bytes()).unwrap();
  }else{
    let content = fs::read_to_string("./views/over.html").unwrap();
    stream.write(format!("HTTP/1.1 200 OK{}", content).as_bytes()).unwrap();
  }
  thread::sleep(Duration::from_secs(3));
  stream.flush().unwrap();
}

fn main() {
  let pool = ThreadPool::new(5);
  let socket = TcpListener::bind("127.0.0.1:8080").unwrap();
  for stream in socket.incoming(){
    pool.execute(||{
      handle_connection(stream.unwrap(), 3);
    })
  }
}
