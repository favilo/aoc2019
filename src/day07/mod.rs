use itertools::Itertools;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{fs, path::Path, str::FromStr, time::Instant};

use crate::intcode::*;

const DAY: u64 = 7;

fn run(p: &Program, input: Vec<isize>) -> Vec<isize> {
    p.clone().interpret_input(input.into_iter())
}

pub fn stage1(p: &Program) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    let mut largest = 0;
    let mut largest_phase = vec![];
    for phases in (0..5).permutations(5) {
        let output = run(&p.clone(), vec![phases[0], 0]);
        let output = run(&p.clone(), vec![phases[1], output[0]]);
        let output = run(&p.clone(), vec![phases[2], output[0]]);
        let output = run(&p.clone(), vec![phases[3], output[0]]);
        let output = run(&p.clone(), vec![phases[4], output[0]]);
        if output[0] > largest {
            largest = output[0];
            largest_phase = phases;
        }
    }

    log::info!("{:?} -> {:?}", largest_phase, largest);
    largest
}

pub fn stage2(p: &Program) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");

    let mut largest = 0;
    let mut largest_phase = vec![];
    let mut last;

    for phases in (5..10).permutations(5) {
        let mut a_p = p.clone();
        let mut b_p = p.clone();
        let mut c_p = p.clone();
        let mut d_p = p.clone();
        let mut e_p = p.clone();

        let output = a_p
            .interpret_to_output(&mut vec![phases[0], 0].into_iter())
            .unwrap();
        let output = b_p
            .interpret_to_output(&mut vec![phases[1], output].into_iter())
            .unwrap();
        let output = c_p
            .interpret_to_output(&mut vec![phases[2], output].into_iter())
            .unwrap();
        let output = d_p
            .interpret_to_output(&mut vec![phases[3], output].into_iter())
            .unwrap();
        let output = e_p
            .interpret_to_output(&mut vec![phases[4], output].into_iter())
            .unwrap();
        last = output;
        loop {
            let output = a_p.interpret_to_output(&mut vec![last].into_iter());
            if output.is_none() {
                break;
            }
            last = output.unwrap();
            let output = b_p.interpret_to_output(&mut vec![last].into_iter());
            if output.is_none() {
                break;
            }
            last = output.unwrap();
            let output = c_p.interpret_to_output(&mut vec![last].into_iter());
            if output.is_none() {
                break;
            }
            last = output.unwrap();
            let output = d_p.interpret_to_output(&mut vec![last].into_iter());
            if output.is_none() {
                break;
            }
            last = output.unwrap();
            let output = e_p.interpret_to_output(&mut vec![last].into_iter());
            if output.is_none() {
                break;
            }
            last = output.unwrap();
        }
        if last > largest {
            largest = last;
            largest_phase = phases;
        }
    }
    log::info!("{:?} -> {:?}", largest_phase, largest);
    largest
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
    let program = Program::new(&input);
    log::debug!("Day {} loading timer: {:?}", DAY, start.elapsed());
    // log::info!("{:?}", input);

    let start = Instant::now();
    let s1 = stage1(&program);
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2(&program);
    log::debug!("Stage 2 linear timer: {:?}", start.elapsed());
    log::info!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_passing() {}
}
