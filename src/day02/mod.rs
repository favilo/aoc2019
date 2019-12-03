use itertools::iproduct;
#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{fs, path::Path, str::FromStr};

use crate::intcode::*;

const DAY: usize = 2;

pub fn stage1(input: &Vec<usize>) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    run(input, 12, 2)
}

fn run(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("run");
    let mut p = Program::new(input);
    p.set_mem(1, noun);
    p.set_mem(2, verb);
    p.interpret()
}

pub fn stage2(input: &Vec<usize>) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");
    for (noun, verb) in iproduct![0..100, 0..100] {
        let output = run(input, noun, verb);
        if output == 19690720 {
            return 100 * noun + verb;
        }
    }
    panic!("Didn't find values")
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day2");
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let s = fs::read_to_string(input_path).expect("Some input needs to exist");
    let input: Vec<usize> = s
        .lines()
        .map(|line| {
            line.split(",")
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect::<Vec<usize>>()
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
    fn test_template() {}
}
