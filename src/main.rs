extern crate clap;

use header_constructor::header_constructor::HeaderConstructor;
use clap::Clap;

mod header_constructor;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short, long)]
    includes: bool,
    #[clap(short, long)]
    defines: bool,
    #[clap(short, long)]
    structs: bool,
    #[clap(long)]
    path: String
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("Hello, world!");
    let opts: Opts = Opts::parse();

    let mut h = HeaderConstructor::new(&opts.path);
    h.generate_header(opts.includes, opts.structs, opts.defines);
}
