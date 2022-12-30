use crate::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // Self & Server are interchangable!
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("running on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // use annotation to break the outer loop from inner loop
        // 'outer: loop {
        //     'inner: loop {}
        // }

        // let tup = (1, "a", {});

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            /*
                            alternatively,
                                1. do &buffer as &[u8]
                                2. let res: &Result<Request, _> = &buffer[..].try_info();
                            */
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => panic!("Faild to parse request: {}", e),
                            };
                        }
                        Err(e) => panic!("Encounter error: {}", e),
                    };
                }
                Err(e) => panic!("Encounter error: {}", e),
            }
        }
    }
}
