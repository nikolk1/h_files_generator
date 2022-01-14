extern crate clap;

use clap::Parser;
use header_constructor::header_constructor::HeaderConstructor;

mod header_constructor;

#[derive(Parser)]
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
    let opts: Opts = Opts::parse();

    let mut h = HeaderConstructor::new(&opts.path);
    h.generate_header(opts.includes, opts.structs, opts.defines);
}
