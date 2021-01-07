use header_constructor::header_constructor::HeaderConstructor;
use std::env;

mod header_constructor;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("Hello, world!");

    let mut h = HeaderConstructor::new("/home/nikol/Documents/h_generator/src/test.c");
    h.generate_header(true, true, true);
}
