use ndarray::{Array, Array1, Array2, Array3, Axis};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{
    fs,
    path::Path,
    time::Instant,
};

const DAY: u64 = 8;

type INPUT = Array3<u8>;

fn parse_image(input: &str, width: usize, height: usize) -> INPUT {
    let input = input.trim();
    let depth = input.len() / (width * height);
    Array::from_shape_vec(
        (depth, height, width),
        input
            .as_bytes()
            .iter()
            // .filter(|&b| *b as char != '\n')
            .map(|&b| (b as char).to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

pub fn stage1(picture: &INPUT) -> usize {
    let layer = picture
        .axis_iter(Axis(0))
        .map(|layer| (Some(layer), layer.iter().filter(|&b| *b == 0).count()))
        .fold(
            (None, 100000),
            |other: (Option<Array2<u8>>, usize), this| {
                if this.1 < other.1 {
                    (Some(this.0.unwrap().into_owned()), this.1)
                } else {
                    other
                }
            },
        )
        .0
        .unwrap();
    layer.iter().filter(|&b| *b == 1).count() * layer.iter().filter(|&b| *b == 2).count()
}

pub fn stage2(picture: &INPUT) -> usize {
    let layer = Array2::from_shape_vec(
        (25, 6),
        picture
            .axis_iter(Axis(2))
            .map(|slice| {
                slice
                    .axis_iter(Axis(1))
                    .map(|v| v.into_owned())
                    .map(|strip: Array1<u8>| {
                        strip.iter().fold(2, |o, b| if o == 2 { *b } else { o })
                    })
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    layer.axis_iter(Axis(1)).for_each(|strip| {
        strip
            .iter()
            .for_each(|&b| if b == 1 { print!("#") } else { print!(" ") });
        println!();
    });
    0
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day8");
    let start = Instant::now();
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let input = fs::read_to_string(input_path).expect("Some input needs to exist");
    let input = parse_image(&input, 25, 6);
    // log::info!("Input: {:?}", input);

    log::debug!("Day 8 loading timer: {:?}", start.elapsed());

    // log::info!("{:?}", input);
    let start = Instant::now();
    let s1 = stage1(&input);
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2(&input);
    log::debug!("Stage 2 timer: {:?}", start.elapsed());
    log::info!("{:?}", s2);
}
