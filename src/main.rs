use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
}

fn main() {
    let _args = Args::parse();
}
