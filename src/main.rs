use clap::Parser;

//4.4.x version
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

//3.2.20 version
// #[derive(Parser)]
// #[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
// struct Cli {
//     #[clap(long)]
//     two: String,
//     #[clap(long)]
//     one: String,
// }

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
