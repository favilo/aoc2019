#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{collections::HashSet, fs, path::Path, time::Instant};

const DAY: u64 = 3;

type Point = (isize, isize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Line {
    Horiz(Point, usize),
    Vert(Point, usize),
}

impl Line {
    fn build(p: Point, s: &String) -> (Self, Point) {
        match s.chars().next().unwrap() {
            'R' => {
                let this = Self::Horiz(p, usize::from_str_radix(&s[1..], 10).unwrap());
                (this, this.end())
            }
            'L' => {
                let d = isize::from_str_radix(&s[1..], 10).unwrap();
                let this = Self::Horiz((p.0 - d, p.1), d as usize);
                (this, this.start())
            }
            'U' => {
                let this = Self::Vert(p, usize::from_str_radix(&s[1..], 10).unwrap());
                (this, this.end())
            }
            'D' => {
                let d = isize::from_str_radix(&s[1..], 10).unwrap();
                let this = Self::Vert((p.0, p.1 - d), d as usize);
                (this, this.start())
            }
            _ => panic!("Doesn't work"),
        }
    }

    fn start(&self) -> Point {
        match self {
            Line::Horiz(p, _) => *p,
            Line::Vert(p, _) => *p,
        }
    }

    fn end(&self) -> Point {
        match self {
            Line::Horiz(p, d) => (p.0 + *d as isize, p.1),
            Line::Vert(p, d) => (p.0, p.1 + *d as isize),
        }
    }

    fn dist(&self) -> usize {
        match self {
            Line::Horiz(_, d) => *d,
            Line::Vert(_, d) => *d,
        }
    }

    fn inside(&self, p: Point) -> bool {
        if self.start().0 == self.end().0 {
            p.0 == self.start().0 && p.1 >= self.start().1 && p.1 <= self.end().1
        } else {
            p.1 == self.start().1 && p.0 >= self.start().0 && p.0 <= self.end().0
        }
    }

    fn other(&self, p: Point) -> Point {
        if p == self.start() {
            self.end()
        } else {
            self.start()
        }
    }
}

#[derive(Debug)]
struct Panel {
    wires: Vec<Vec<Line>>,
    idx: usize,
    last_point: Point,
}

impl Panel {
    fn new() -> Self {
        Self {
            wires: vec![vec![]],
            idx: 0,
            last_point: (0, 0),
        }
    }

    fn construct_wire(mut self, wire: Vec<String>) -> Self {
        for w in wire {
            self.add_segment(&w);
        }
        self.next_wire();
        self
    }

    fn add_segment(&mut self, s: &String) {
        let (line, p) = Line::build(self.last_point, s);
        self.wires[self.idx].push(line);
        self.last_point = p;
    }

    fn next_wire(&mut self) {
        self.idx += 1;
        self.last_point = (0, 0);
        self.wires.push(vec![]);
    }

    fn sorted_verts(&self, idx: usize) -> Vec<Line> {
        let mut v = self.wires[idx]
            .iter()
            .cloned()
            .filter(|l| match l {
                Line::Horiz(_, _) => false,
                Line::Vert(_, _) => true,
            })
            .collect::<Vec<Line>>();
        v.sort_by(|a, b| Ord::cmp(&a.start().0, &b.start().0));
        v
    }

    fn sorted_horiz_ends(&self, idx: usize) -> Vec<Line> {
        let mut v = self.wires[idx]
            .iter()
            .cloned()
            .filter(|l| match l {
                Line::Horiz(_, _) => true,
                Line::Vert(_, _) => false,
            })
            .collect::<Vec<Line>>();
        v.sort_by(|a, b| Ord::cmp(&a.end().0, &b.end().0));
        v
    }

    fn sorted_horiz_starts(&self, idx: usize) -> Vec<Line> {
        let mut v = self.wires[idx]
            .iter()
            .cloned()
            .filter(|l| match l {
                Line::Horiz(_, _) => true,
                Line::Vert(_, _) => false,
            })
            .collect::<Vec<Line>>();
        v.sort_by(|a, b| Ord::cmp(&a.start().0, &b.start().0));
        v
    }

    fn intersections(&self, from: usize, to: usize) -> impl Iterator<Item = Point> {
        let mut checking: HashSet<Line> = HashSet::new();
        let mut intersections = Vec::new();

        let starts = self.sorted_horiz_starts(from);
        let ends = self.sorted_horiz_ends(from);
        let verts = self.sorted_verts(to);
        let (mut start_idx, mut end_idx, mut vert_idx) = (0, 0, 0);

        loop {
            if vert_idx >= verts.len() || start_idx >= starts.len() {
                break;
            }
            let (start_x, end_x, vert_x) = (
                starts[start_idx].start().0,
                ends[end_idx].end().0,
                verts[vert_idx].start().0,
            );
            let min = *[start_x, end_x, vert_x].iter().min().to_owned().unwrap();
            if min == start_x {
                checking.insert(starts[start_idx]);
                start_idx += 1;
            } else if min == end_x {
                checking.remove(&ends[end_idx]);
                end_idx += 1;
            } else {
                intersections.append(&mut check_vert(&checking, verts[vert_idx]));
                vert_idx += 1;
            }
        }

        intersections.into_iter()
    }

    fn steps_to(&self, p: Point, idx: usize) -> usize {
        let mut last_point = (0, 0);
        let mut dist = 0;
        for l in self.wires[idx].to_vec() {
            if l.inside(p) {
                dist += ((p.0 - last_point.0).abs() + (p.1 - last_point.1).abs()) as usize;
                break;
            }
            dist += l.dist();
            last_point = l.other(last_point);
        }

        dist
    }
}

fn check_vert(checking: &HashSet<Line>, vert: Line) -> Vec<Point> {
    checking
        .iter()
        .cloned()
        .filter(|h| vert.clone().start().1 <= h.clone().start().1 && vert.end().1 >= h.start().1)
        .map(|h| (vert.clone().start().0, h.clone().start().1))
        .collect()
}

fn manhattan_distance(p: Point) -> usize {
    (p.0.abs() + p.1.abs()) as usize
}

pub fn stage1(input: &Vec<Vec<String>>) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    let panel = Panel::new();
    let panel = input
        .iter()
        .fold(panel, |p, v| p.construct_wire(v.to_vec()));
    let last = panel.intersections(0, 1).chain(panel.intersections(1, 0));
    // log::info!("{:?}", last.collect::<Vec<Point>>());
    last.map(manhattan_distance).min().unwrap()
}

pub fn stage2(input: &Vec<Vec<String>>) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");

    let panel = Panel::new();
    let panel = input
        .iter()
        .fold(panel, |p, v| p.construct_wire(v.to_vec()));
    let last: Vec<Point> = panel
        .intersections(0, 1)
        .chain(panel.intersections(1, 0))
        .collect();

    let wire1 = last.iter().map(|p| panel.steps_to(*p, 0));
    let wire2 = last.iter().map(|p| panel.steps_to(*p, 1));
    // log::info!("{:?}", wire1.zip(wire2).collect::<Vec<(usize, usize)>>());
    wire1.zip(wire2).map(|(a, b)| a + b).min().unwrap()
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day1");
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let s = fs::read_to_string(input_path).expect("Some input needs to exist");
    let input: Vec<Vec<String>> = s
        .lines()
        .map(|line| {
            line.split(",")
                .map(&str::to_string)
                .collect::<Vec<String>>()
        })
        .collect();
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
    fn test_line_build() {
        assert_eq!((-10, 0), Line::build((0, 0), &"L10".to_string()).0.start());
        assert_eq!((10, 0), Line::build((0, 0), &"R10".to_string()).0.end());
        assert_eq!((0, 10), Line::build((0, 0), &"U10".to_string()).0.end());
        assert_eq!((0, -10), Line::build((0, 0), &"D10".to_string()).0.start());
    }

    #[test]
    fn test_panel_sorted_verts() {
        let mut p = Panel::new();
        let p = p.construct_wire(vec![
            "L10".to_string(),
            "U13".to_string(),
            "R20".to_string(),
            "D1".to_string(),
            "L30".to_string(),
        ]);
        assert_eq!(
            vec![-10, 10],
            p.sorted_verts(0)
                .iter()
                .map(Line::start)
                .map(|p| p.0)
                .collect::<Vec<isize>>()
        );
    }

    #[test]
    fn test_panel_sorted_horiz_ends() {
        let mut p = Panel::new();
        let p = p.construct_wire(vec![
            "L10".to_string(),
            "U13".to_string(),
            "R20".to_string(),
            "D1".to_string(),
            "L30".to_string(),
            "U30".to_string(),
            "L30".to_string(),
        ]);
        println!("{:?}", p.sorted_horiz_ends(0));
        assert_eq!(
            vec![-20, 0, 10, 10],
            p.sorted_horiz_ends(0)
                .iter()
                .map(Line::end)
                .map(|p| p.0)
                .collect::<Vec<isize>>()
        );
    }

    #[test]
    fn test_panel_sorted_horiz_starts() {
        let mut p = Panel::new();
        let p = p.construct_wire(vec![
            "L10".to_string(),
            "U13".to_string(),
            "R20".to_string(),
            "D1".to_string(),
            "L30".to_string(),
            "U30".to_string(),
            "L30".to_string(),
        ]);
        println!("{:?}", p.sorted_horiz_starts(0));
        assert_eq!(
            vec![-50, -20, -10, -10],
            p.sorted_horiz_starts(0)
                .iter()
                .map(Line::start)
                .map(|p| p.0)
                .collect::<Vec<isize>>()
        );
    }
}
