#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{fs, path::Path, str::FromStr};

const DAY: u64 = 1;

pub fn stage1(input: &Vec<u64>) -> u64 {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    input.iter().map(|n| n / 3 - 2).sum()
}

fn fuel(n: &i32) -> i32 {
    n / 3 - 2
}

fn module_total_fuel(m: &u64) -> u64 {
    let mut mass = *m;
    let mut total_fuel = 0;
    loop {
        let fuel_mass = fuel(&(mass as i32));
        if fuel_mass <= 0 {
            break;
        }
        total_fuel += fuel_mass as u64;
        // println!("{:?}", total_fuel);
        mass = fuel_mass as u64;
    }
    total_fuel
}

pub fn stage2(input: &Vec<u64>) -> u64 {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");
    let v = input.iter().map(module_total_fuel);
    v.collect::<Vec<u64>>().iter().sum::<u64>()
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day1");
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let s = fs::read_to_string(input_path).expect("Some input needs to exist");
    let input: Vec<u64> = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(u64::from_str)
                .map(Result::unwrap)
                .collect::<Vec<u64>>()
        })
        .flatten()
        .collect();
    let s1 = stage1(&input);
    log::info!("{:?}", s1);
    let s2 = stage2(&input);
    log::info!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_fuel() {
        assert_eq!(module_total_fuel(&1969), 966);
    }
}
