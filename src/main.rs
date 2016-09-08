extern crate futures;

mod input;
mod timeout;

use futures::Future;
use input::ReadLine;
use std::io;
use timeout::Timeout;

fn main() {
    match read_name() {
        Err(_) => println!("Hello, whatever your name is."),
        Ok(name) => println!("Hello, {}!", name.trim()),
    }
}

fn read_name() -> io::Result<String> {
    use std::time::Duration;

    let result = ReadLine::new()
        .select(Timeout::new(Duration::from_secs(5), || {
            io::Error::new(io::ErrorKind::Other, "timeout elapsed".to_string())
        }))
        .wait();

    match result {
        Ok((name, _)) => Ok(name),
        Err((e, _)) => Err(e),
    }
}
