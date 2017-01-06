#![warn(missing_docs)]
extern crate futures;
extern crate tokio_core;

use futures::IntoFuture;
use tokio_core::reactor::{Core, Handle};

pub fn current() -> Handle {
    unimplemented!();
}

pub fn is_running() -> bool {
    unimplemented!();
}

pub fn run<F: FnOnce() -> R, R: IntoFuture>(f: F)
    -> Result<R::Item, R::Error>
{
    let mut lp = Core::new().expect("create loop");
    lp.run(futures::lazy(f))
}
