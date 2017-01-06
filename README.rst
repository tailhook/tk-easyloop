===============
Tokio Easy Loop
===============

:Status: Beta
:Documentation: http://docs.rs/tk-easyloop/

A main loop wrapper around tokio to provide thread-local
loop which:

* Avoids padding a ``Handle`` in to every function
* Mostly avoids common error: ``thread 'foo' panicked at 'no Task is
  currently running'``, by providing convenient `run` function for all your
  code involving futures


Example
=======

.. code-block:: rust

    extern crate futures;
    extern crate tk_easyloop;

    use std::time::Duration;
    use tk_easyloop::{run, timeout};

    fn main() {
        run(|| {
            // should return some future, let's use a timeout
            timeout(Duration::new(1, 0))
        }).unwrap();
    }


Multi-threaded Example
======================

This crate uses thread-local storage for storing loop, but it doesn't
mean multi-treading doesn't work. Multiple threads can be used too.

.. code-block:: rust

    extern crate tk_easyloop;
    use std::thread;
    use std::time::Duration;
    use tk_easyloop::{run, timeout};

    fn main() {
        let mut threads = Vec::new();
        for thread_no in 0..10 {
            threads.push(thread::spawn(move || {
                run(|| {
                    timeout(Duration::new(1, 0))
                })
            }))
        }
        for t in threads {
            t.join().unwrap().unwrap();
        }
    }

See ``examples/multi-threaded.rs`` for more comprehensive example.


License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

