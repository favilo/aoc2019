#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{fs, path::Path, str::FromStr, time::Instant};

use crate::intcode::*;

const DAY: usize = 2;

pub fn stage1(input: &Vec<isize>) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    run(input, 12, 2)
}

fn run(input: &Vec<isize>, noun: isize, verb: isize) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("run");
    let mut p = Program::new(input);
    p.load_input(noun, verb);
    p.interpret()
}

#[cfg(feature = "include_slow")]
pub fn stage2(input: &Vec<isize>) -> isize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");
    let verb = 0;
    let noun = {
        let mut final_noun = 0;
        for noun in 0..100 {
            let output = run(input, noun, verb);
            if output > 19690720 {
                final_noun = noun - 1;
                break;
            }
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
        final_noun
    };

    for verb in 0..100 {
        let output = run(input, noun, verb);
        log::trace!("{}, {} => {}", noun, verb, output);
        if output == 19690720 {
            return 100 * noun + verb;
        }
    }
    panic!("Didn't find values")
}

pub fn stage2_linear(input: &Vec<isize>) -> isize {
    let n0 = run(input, 0, 0);
    let n1 = run(input, 1, 0);
    let v1 = run(input, 0, 1);
    let n = n1 - n0;
    let v = v1 - n0;

    let goal = 19690720;
    let noun = (goal - n0) / n;
    let verb = (goal - n0 - n * noun) / v;
    log::trace!("{}, {}", noun, verb);
    100 * noun + verb
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day2");
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

    log::debug!("Day {} load: {:?}", DAY, start.elapsed());

    let start = Instant::now();
    let s1 = stage1(&input);
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2_linear(&input);
    log::debug!("Stage 2 linear timer: {:?}", start.elapsed());
    log::info!("{:?}", s2);

    #[cfg(feature = "include_slow")]
    {
        let start = Instant::now();
        let s2 = stage2(&input);
        log::debug!("Stage 2 old timer: {:?}", start.elapsed());
        log::info!("{:?}", s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let input_path = Path::new("src")
            .join(format!("day{:02}", DAY))
            .join("input");
        log::debug!("Opening file {:?}", input_path);
        let s = fs::read_to_string(input_path).expect("Some input needs to exist");
        let input: Vec<isize> = s
            .lines()
            .map(|line| {
                line.split(",")
                    .map(isize::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .collect();
        let n0 = run(&input, 0, 0);
        let n1 = run(&input, 1, 0);
        let v1 = run(&input, 0, 1);
        let n = n1 - n0;
        let v = v1 - n0;
        // assert_eq!(n, 856118);
        let noun = 23;
        let verb = 47;
        assert_eq!(run(&input, noun, verb), n * noun + v * verb + n0);

        assert_eq!(1, 1);
    }
}
