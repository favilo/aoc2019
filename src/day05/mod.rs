#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{
    fs,
    path::Path,
    str::FromStr,
    time::Instant,
};

use crate::intcode::*;

const DAY: u64 = 5;

fn run(input: &Vec<isize>, i: Vec<isize>) -> Vec<isize> {
    let mut p = Program::new(input);
    p.interpret_input(i.into_iter())
}

pub fn stage1(input: &Vec<isize>) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    let output = run(input, vec![1]);
    log::info!("{:?}", output);

    *output.last().unwrap()
}

pub fn stage2(input: &Vec<isize>) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");
    // run(input)
    let output = run(input, vec![5]);
    log::info!("{:?}", output);
    *output.last().unwrap()
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day1");

    let start = Instant::now();
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let input = &fs::read_to_string(input_path).expect("Some input needs to exist");

    let input = input
        .lines()
        .map(|s| s.split(',').map(isize::from_str).map(Result::unwrap))
        .flatten()
        .collect::<Vec<isize>>();
    log::debug!("Day {} loading timer: {:?}", DAY, start.elapsed());
    // log::info!("{:?}", input);

    let start = Instant::now();
    let s1 = stage1(&input);
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2(&input);
    log::debug!("Stage 2 linear timer: {:?}", start.elapsed());
    log::info!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_passing() {}
}
