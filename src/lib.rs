#![warn(missing_docs)]

extern crate futures;
extern crate tokio_core;
#[macro_use] extern crate scoped_tls;

use std::time::Duration;

use futures::IntoFuture;
use tokio_core::reactor::{Core, Handle, Timeout, Interval};


scoped_thread_local! {
    static HANDLE: Handle
}

pub fn handle() -> Handle {
    HANDLE.with(|handle| handle.clone())
}

pub fn is_running() -> bool {
    HANDLE.is_set()
}

pub fn run<F: FnOnce() -> R, R: IntoFuture>(f: F)
    -> Result<R::Item, R::Error>
{
    let mut lp = Core::new().expect("create loop");
    HANDLE.set(&lp.handle(), || {
        lp.run(futures::lazy(f))
    })
}

pub fn timeout(dur: Duration) -> Timeout {
    HANDLE.with(|handle| {
        Timeout::new(dur, handle).unwrap()
    })
}

pub fn interval(dur: Duration) -> Interval {
    HANDLE.with(|handle| {
        Interval::new(dur, handle).unwrap()
    })
}
