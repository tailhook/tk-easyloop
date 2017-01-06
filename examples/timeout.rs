extern crate futures;
extern crate tk_easyloop;

use std::time::Duration;
use futures::Future;
use tk_easyloop::{run, timeout};

fn main() {

    run(|| {
        println!("Sleeping 1 second");
        timeout(Duration::new(1, 0))
        .and_then(|()| {
            println!("Done");
            Ok(())
        })
    }).unwrap();
}
