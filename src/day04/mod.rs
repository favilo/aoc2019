use itertools::Itertools;
use std::time::Instant;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

fn digits(x: usize) -> impl DoubleEndedIterator<Item = usize> {
    let digit_count = (x as f32).log10() as usize + 1;
    (0..digit_count).map(move |exp| x / 10usize.pow(exp as u32) % 10)
}

#[allow(dead_code)]
fn is_increasing(_x: usize) -> bool {
    //digits(x).tuple_windows()
    true
}

fn next_passing(&i: &usize) -> usize {
    let mut first_bad = None;
    let first_digit = digits(i).last().unwrap();
    let next_passing = digits(i)
        .rev()
        .tuple_windows()
        .map(|(a, b)| {
            if first_bad.is_some() {
                first_bad.unwrap()
            } else if a > b {
                first_bad = Some(a);
                a
            } else {
                b
            }
        })
        .fold(first_digit, |r, m| r * 10 + m);
    next_passing
}

fn matches(&i: &usize) -> Result<(), usize> {
    let mut last = None;
    let mut double = false;

    for c in digits(i).rev() {
        if last.is_some() && last.unwrap() > c {
            let diff = next_passing(&i) - i;
            return Err(diff);
        }

        if Some(c) == last {
            double = true;
        }
        last = Some(c);
    }

    if double {
        Ok(())
    } else {
        Err(1)
    }
}

fn matches_2(&i: &usize) -> Result<(), usize> {
    let mut runs = Vec::with_capacity(10);

    for c in digits(i) {
        if runs.last().is_none() {
            runs.push((c, 1));
            continue;
        }

        if runs.last().is_some() && runs.last().unwrap().0 < c {
            let diff = next_passing(&i) - i;
            return Err(diff);
        }
        if c == runs.last().unwrap().0 {
            let (ch, count) = runs.last().unwrap();
            let idx = runs.len() - 1;
            runs[idx] = (*ch, count + 1)
        } else {
            runs.push((c, 1));
        }
    }

    if runs
        .into_iter()
        .filter(|c| c.1 == 2)
        .collect::<Vec<(usize, usize)>>()
        .len()
        > 0
    {
        Ok(())
    } else {
        Err(1)
    }
}

pub fn stage1(_input: &str) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");

    // let range = 108457..=562041;
    let mut count = 0;

    let mut x = 108457;
    while x <= 562041 {
        let m = matches(&x);
        if m.is_ok() {
            count += 1;
            x += 1;
        } else {
            let diff = m.err().unwrap();
            x += diff;
        }
    }
    count
}

pub fn stage2(_input: &str) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");

    // let range = 108457..=562041;

    let mut count = 0;

    let mut x = 108457;
    while x <= 562041 {
        let m = matches_2(&x);
        if m.is_ok() {
            count += 1;
            x += 1;
        } else {
            let diff = m.err().unwrap();
            x += diff;
        }
    }
    count
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day1");

    // log::info!("{:?}", input);
    let start = Instant::now();
    let s1 = stage1("");
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2("");
    log::debug!("Stage 2 linear timer: {:?}", start.elapsed());
    log::info!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_passing() {
        assert_eq!(111111, next_passing(&108457));
        assert_eq!(111119, next_passing(&111119));
        assert_eq!(111111, next_passing(&111111));
        assert_eq!(199999, next_passing(&199999));
        assert_eq!(222222, next_passing(&200000));
    }
}
