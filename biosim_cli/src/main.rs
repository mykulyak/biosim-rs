use clap::Parser;

use biosim_core::Simulator;

#[derive(Parser, Debug)]
#[clap(
    author = "Andriy Mykulyak",
    version,
    about = "Biological evolution simulator written in Rust"
)]
struct Args {
    #[arg(short = 'c', long = "config-file", value_parser)]
    config_file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut simulator = Simulator::new();

    simulator.run();

    println!("{:?}", args.config_file);
    println!("{:?}", simulator);
    println!("Hello, world!");
}
