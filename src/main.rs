use clap::{App, Arg};
use lazy_static::lazy_static;

use std::{collections::HashMap, str::FromStr, time::Instant};

mod day01;
mod day02;
mod intcode;

lazy_static! {
    static ref DAYS: HashMap<u8, fn()> = {
        let mut m = HashMap::<u8, fn()>::new();
        m.insert(1, day01::run_day);
        m.insert(2, day02::run_day);
        m
    };
}

fn setup() {
    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    env_logger::init();
}

fn main() {
    let start = Instant::now();
    setup();
    let app = App::new("AoC 2019")
        .version("1.0")
        .author("Favil Orbedios <favilo@gmail.com>")
        .about("Runs the Advent of Code 2019 challenges")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Run all days"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .multiple(true)
                .takes_value(true)
                .number_of_values(1)
                .help("Which day to run"),
        );
    let matches = app.get_matches();
    let days: Vec<u8> = {
        if matches.is_present("all") {
            (1..=12).collect()
        } else {
            let values = matches.values_of("day");
            if values.is_none() {
                vec![]
            } else {
                values
                    .unwrap()
                    .map(u8::from_str)
                    .map(Result::unwrap)
                    .collect()
            }
        }
    };

    log::debug!("Days to cover: {:?}", days);
    for day in days {
        if DAYS.contains_key(&day) {
            let start = Instant::now();
            log::debug!("Starting day {}", day);
            DAYS[&day]();
            log::debug!("Day {} timing: {:?}", day, start.elapsed());
        }
    }
    log::info!("Timing: {:?}", start.elapsed());
    #[cfg(feature = "profiler")]
    {
        let output = "./profile.json";
        println!("Writing output to {}", output);

        thread_profiler::write_profile(output);
    }
}
