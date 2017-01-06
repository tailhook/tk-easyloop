extern crate futures;
extern crate tokio_core;
extern crate tk_easyloop;

use std::net::{TcpListener as StdListener, SocketAddr};
use std::thread;
use std::env;

use futures::stream::Stream;
use tokio_core::net::TcpListener;
use tk_easyloop::{run, handle};


fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:7777".to_string());
    let addr = addr.parse::<SocketAddr>().expect("parse address");

    let socket = StdListener::bind(addr).expect("bind socket");
    let mut threads = Vec::new();
    for thread_no in 0..10 {
        let lst = socket.try_clone().expect("cloning bind socket");
        threads.push(thread::spawn(move || {
            run(|| {
                TcpListener::from_listener(lst, &addr, &handle())
                .expect("listener created")
                .incoming()
                .for_each(|(_sock, addr)| {
                    println!("Accepted {} in thread {}", addr, thread_no);
                    Ok(())
                })
            })
        }))
    }
    for t in threads {
        t.join().expect("thread never joins").expect("accept never fails");
    }
}
