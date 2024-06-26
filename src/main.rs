use gittogether::period::{get_recent_one_month, parse_from_input};
use gittogether::process;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repo: String,
    period: Option<String>,

    #[arg(short, long, default_value = "compact")]
    style: String,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    let periods = if let Some(period_str) = args.period {
        parse_from_input(&period_str)
    } else {
        get_recent_one_month()
    };

    let svg = process(&args.repo, periods, &args.style).await;
    svg::save("image.svg", &svg).unwrap();
}
