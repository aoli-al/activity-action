use activity_action::period::{get_recent10_days, parse_from_input};
use activity_action::process;
use std::{array, env};
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
        get_recent10_days()
    };

    let svg = process(&args.repo, periods, &args.style).await;
    svg::save("image.svg", &svg).unwrap();
}
