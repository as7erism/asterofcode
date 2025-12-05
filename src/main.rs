#![feature(btree_cursors)]

use core::panic;
use std::{env, fs, time::Instant};

use clap::Parser;
use dotenvy::dotenv;

mod year2025;

const URL: &str = "https://adventofcode.com";

/// collection of aster's advent of code solutions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..13))]
    day: u8,

    #[arg(
        short,
        long,
        default_value_t = 2025,
        value_parser = clap::value_parser!(u16).range(2025..2026)
    )]
    year: u16,

    #[arg(
        short,
        long,
        value_parser = clap::value_parser!(u8).range(1..3)
    )]
    part: Option<u8>,

    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    session_id: Option<String>,
}

fn time<F>(f: F)
where
    F: FnOnce(),
{
    let timer = Instant::now();
    f();
    eprintln!("took {} μs", timer.elapsed().as_micros());
}

trait Solution {
    fn part_one(input: &str);

    fn part_two(input: &str);

    fn run(part: Option<u8>, input: &str) {
        match part {
            Some(p) => {
                if p == 1 {
                    time(|| Self::part_one(input));
                } else {
                    time(|| Self::part_two(input));
                }
            }
            None => {
                time(|| Self::part_one(input));
                time(|| Self::part_two(input));
            }
        }
    }
}

fn main() {
    let _ = dotenv();
    let args = Args::parse();

    let input = if let Some(input) = args.input {
        input
    } else if let Some(path) = args.file {
        fs::read_to_string(&path).unwrap_or_else(|_| panic!("error reading input file {path}"))
    } else {
        let session_id = args.session_id.unwrap_or_else(|| {
            env::var("SESSION_ID").expect("no input given and no session id to fetch input given")
        });

        reqwest::blocking::Client::new()
            .get(format!("{URL}/{}/day/{}/input", args.year, args.day))
            .header("Cookie", format!("session={session_id}"))
            .send()
            .expect("no input given and failed to complete http request for input")
            .error_for_status()
            .expect("no input given and http request failed\n\tis your session id valid?")
            .text()
            .expect("no input given and failed to parse http response")
    };

    match args.year {
        2025 => year2025::run(args.day, args.part, &input),
        _ => panic!("haven't done this year yet..."),
    }
}
