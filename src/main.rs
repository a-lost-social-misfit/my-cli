use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// 名前を入力
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
