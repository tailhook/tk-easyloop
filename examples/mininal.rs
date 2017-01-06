extern crate tk_easyloop;

use std::io;
use tk_easyloop::run;

fn main() {

    run(|| {
        Ok::<_, io::Error>(())
    }).unwrap();
}
