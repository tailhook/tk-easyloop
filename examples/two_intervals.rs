extern crate futures;
extern crate tk_easyloop;

use std::time::Duration;
use futures::{Stream, Future};
use tk_easyloop::{run_forever, interval, spawn};

fn main() {

    run_forever(|| {
        spawn(interval(Duration::new(1, 0))
            .for_each(|()| {
                println!("1 sec interval");
                Ok(())
            }).map_err(|_| ()));
        spawn(interval(Duration::from_millis(500))
            .for_each(|()| {
                println!("Half second interval");
                Ok(())
            }).map_err(|_| ()));
        Ok::<_, ()>(())
    }).unwrap();
}
