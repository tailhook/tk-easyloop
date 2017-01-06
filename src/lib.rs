//! A main loop wrapper around tokio to provide thread-local
//! loop which:
//!
//! * Avoids padding a `Handle` in to every function
//!
//! * Mostly avoids common error: `thread 'foo' panicked at 'no Task is
//! currently running'`, by providing convenient `run` function for all your
//! code involving futures
//!
//!
//! # Example
//!
//! ```no_run
//! extern crate futures;
//! extern crate tk_easyloop;
//!
//! use std::time::Duration;
//! use tk_easyloop::{run, timeout};
//!
//! fn main() {
//!     run(|| {
//!         // should return some future, let's use a timeout
//!         timeout(Duration::new(1, 0))
//!     }).unwrap();
//! }
//! ```
//!
//! # Multi-threaded Example
//!
//! This crate uses thread-local storage for storing loop, but it doesn't
//! mean multi-treading doesn't work. Multiple threads can be used too.
//!
//! ```
//! extern crate tk_easyloop;
//! use std::thread;
//! use std::time::Duration;
//! use tk_easyloop::{run, timeout};
//!
//! fn main() {
//!     let mut threads = Vec::new();
//!     for thread_no in 0..10 {
//!         threads.push(thread::spawn(move || {
//!             run(|| {
//!                 timeout(Duration::new(1, 0))
//!             })
//!         }))
//!     }
//!     for t in threads {
//!         t.join().unwrap().unwrap();
//!     }
//! }
//! ```
//!
//! See ``examples/multi-threaded.rs`` for more comprehensive example.
#![warn(missing_docs)]

extern crate futures;
extern crate tokio_core;
#[macro_use] extern crate scoped_tls;

use std::time::{Duration, Instant};

use futures::IntoFuture;
use tokio_core::reactor::{Core, Handle, Timeout, Interval};


scoped_thread_local! {
    static HANDLE: Handle
}

/// Returns current loop handle
///
/// This only works if running inside the `run()` function of the main loop
///
/// # Panics
///
/// This function panics if there is no currently running loop (i.e. this
/// function is not running from the inside of `run()`.
pub fn handle() -> Handle {
    HANDLE.with(|handle| handle.clone())
}

/// Returns `true` if there is an event loop currently running
///
/// This basically returns `false` if and only if `handle()` would panic
pub fn is_running() -> bool {
    HANDLE.is_set()
}

/// Run the main loop and initialize it by running a function
///
/// This is basically a shortcut for:
///
/// ```ignore
/// let mut lp = Core::new().expect("create loop");
/// lp.run(futures::lazy(f))
/// ```
///
/// But also initializes thread-local loop handle for the time of loop run
pub fn run<F: FnOnce() -> R, R: IntoFuture>(f: F)
    -> Result<R::Item, R::Error>
{
    let mut lp = Core::new().expect("create loop");
    HANDLE.set(&lp.handle(), || {
        lp.run(futures::lazy(f))
    })
}

/// Create a timeout tied to the current loop
///
/// This is a shortcut for:
///
/// ```ignore
/// Timeout::new(dur, &handle()).unwrap()
/// ```
///
/// # Panics
///
/// When no loop is running (`handle()` panics)
///
/// (Note: while we technically `unwrap()` constructor it never fails in
/// current tokio)
pub fn timeout(dur: Duration) -> Timeout {
    HANDLE.with(|handle| {
        Timeout::new(dur, handle).unwrap()
    })
}

/// Create a timeout tied to the current loop
///
/// This is a shortcut for:
///
/// ```ignore
/// Timeout::new_at(instant, &handle()).unwrap()
/// ```
///
/// # Panics
///
/// When no loop is running (`handle()` panics)
///
/// (Note: while we technically `unwrap()` constructor it never fails in
/// current tokio)
pub fn timeout_at(instant: Instant) -> Timeout {
    HANDLE.with(|handle| {
        Timeout::new_at(instant, handle).unwrap()
    })
}

/// Create an interval tied to the current loop
///
/// This is a shortcut for:
///
/// ```ignore,
/// Interval::new(instant, &handle()).unwrap()
/// ```
///
/// # Panics
///
/// When no loop is running (`handle()` panics)
///
/// (Note: while we technically `unwrap()` constructor it never fails in
/// current tokio)
pub fn interval(dur: Duration) -> Interval {
    HANDLE.with(|handle| {
        Interval::new(dur, handle).unwrap()
    })
}
