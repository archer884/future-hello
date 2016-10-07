extern crate futures;

use std::io;

fn main() {
    match read_name() {
        Err(_) => println!("Hello, whatever your name is."),
        Ok(name) => println!("Hello, {}!", name.trim()),
    }
}

fn read_name() -> io::Result<String> {
    use futures::Future;
    use std::thread;
    use std::time::Duration;

    let timeout = {
        let (c, p) = futures::oneshot();
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(5));
            c.complete(Err(io::Error::new(
                io::ErrorKind::Other,
                "Timeout elapsed"
            )));
        });

        p
    };

    let input = {
        let (c, p) = futures::oneshot();
        thread::spawn(|| {
            use std::io::BufRead;

            let input = io::stdin();
            let mut input = input.lock();
            let mut buf = String::new();

            c.complete(input.read_line(&mut buf).map(|_| buf));
        });

        p
    };

    match input.select(timeout).wait() {
        Err(_) => unreachable!(),
        Ok((complete, _)) => complete,
    }
}
