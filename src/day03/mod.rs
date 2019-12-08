use sorted_vec::SortedVec;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{
    cmp::{min, Ord, Ordering, PartialEq},
    collections::HashSet,
    fs,
    path::Path,
    time::Instant,
};

const DAY: u64 = 3;

type Point = (isize, isize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// enum Line {
//     Horiz(Point, usize),
//     Vert(Point, usize),
// }

struct Line {
    vert: bool,
    p: Point,
    d: usize,
}

impl Line {
    fn build(p: Point, s: &String) -> (Self, Point) {
        match s.chars().next().unwrap() {
            'R' => {
                let this = Self {
                    vert: false,
                    p,
                    d: usize::from_str_radix(&s[1..], 10).unwrap(),
                };
                (this, this.end())
            }
            'L' => {
                let d = isize::from_str_radix(&s[1..], 10).unwrap();
                let this = Self {
                    vert: false,
                    p: (p.0 - d, p.1),
                    d: d as usize,
                };
                (this, this.start())
            }
            'U' => {
                let this = Self {
                    vert: true,
                    p,
                    d: usize::from_str_radix(&s[1..], 10).unwrap(),
                };
                (this, this.end())
            }
            'D' => {
                let d = isize::from_str_radix(&s[1..], 10).unwrap();
                let this = Self {
                    vert: true,
                    p: (p.0, p.1 - d),
                    d: d as usize,
                };
                (this, this.start())
            }
            _ => panic!("Doesn't work"),
        }
    }

    #[inline]
    fn start(&self) -> Point {
        self.p
    }

    #[inline]
    fn end(&self) -> Point {
        match self.vert {
            false => (self.p.0 + self.d as isize, self.p.1),
            true => (self.p.0, self.p.1 + self.d as isize),
        }
    }

    #[inline]
    fn dist(&self) -> usize {
        self.d
    }

    #[inline]
    fn inside(&self, p: Point) -> bool {
        if self.start().0 == self.end().0 {
            p.0 == self.start().0 && p.1 >= self.start().1 && p.1 <= self.end().1
        } else {
            p.1 == self.start().1 && p.0 >= self.start().0 && p.0 <= self.end().0
        }
    }

    #[inline]
    fn other(&self, p: Point) -> Point {
        if p == self.start() {
            self.end()
        } else {
            self.start()
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Start(Line);

impl Ord for Start {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.start().cmp(&other.0.start())
    }
}

impl PartialOrd for Start {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct End(Line);

impl Ord for End {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.end().cmp(&other.0.end())
    }
}

impl PartialOrd for End {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Panel {
    wires: Vec<Vec<Line>>,
    idx: usize,
    last_point: Point,
    starts: Vec<Vec<Start>>,
    ends: Vec<Vec<End>>,
    verts: Vec<Vec<Start>>,
}

impl Panel {
    fn new() -> Self {
        Self {
            wires: vec![vec![]],
            idx: 0,
            last_point: (0, 0),
            starts: vec![Vec::with_capacity(1000), Vec::with_capacity(1000)],
            ends: vec![Vec::with_capacity(1000), Vec::with_capacity(1000)],
            verts: vec![Vec::with_capacity(1000), Vec::with_capacity(1000)],
        }
    }

    #[inline]
    fn construct_wire(mut self, wire: Vec<String>) -> Self {
        #[cfg(feature = "profiler")]
        profile_scope!("construct_wire");
        let mut verts = SortedVec::new();
        let mut starts = SortedVec::new();
        let mut ends = SortedVec::new();

        for w in wire {
            self.add_segment(&w, &mut verts, &mut starts, &mut ends);
        }
        self.verts[self.idx].append(&mut verts.to_vec());
        self.starts[self.idx].append(&mut starts.to_vec());
        self.ends[self.idx].append(&mut ends.to_vec());

        self.next_wire();
        self
    }

    #[inline]
    fn add_segment(
        &mut self,
        s: &String,
        verts: &mut SortedVec<Start>,
        starts: &mut SortedVec<Start>,
        ends: &mut SortedVec<End>,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("add_segment");
        let (line, p) = Line::build(self.last_point, s);
        self.wires[self.idx].push(line);
        if line.vert {
            verts.insert(Start(line)).unwrap();
        } else {
            starts.insert(Start(line)).unwrap();
            ends.insert(End(line)).unwrap();
        }
        self.last_point = p;
    }

    #[inline]
    fn next_wire(&mut self) {
        self.idx += 1;
        self.last_point = (0, 0);
        self.wires.push(vec![]);
    }

    fn sorted_verts(&self, idx: usize) -> Vec<Line> {
        self.verts[idx].iter().map(|Start(l)| l.clone()).collect()
    }

    fn sorted_horiz_ends(&self, idx: usize) -> Vec<Line> {
        #[cfg(feature = "profiler")]
        profile_scope!("sorted_horiz_ends");
        self.ends[idx].iter().map(|End(l)| l.clone()).collect()
    }

    fn sorted_horiz_starts(&self, idx: usize) -> Vec<Line> {
        #[cfg(feature = "profiler")]
        profile_scope!("sorted_horiz_starts");
        self.starts[idx].iter().map(|Start(l)| l.clone()).collect()
    }

    fn intersections(&self, from: usize, to: usize) -> impl Iterator<Item = Point> {
        #[cfg(feature = "profiler")]
        profile_scope!("intersections");
        let mut checking: HashSet<Line> = HashSet::with_capacity(1000);
        let mut intersections = Vec::with_capacity(1000);

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
            // let min = *[start_x, end_x, vert_x].iter().min().to_owned().unwrap();
            let min = min(start_x, min(end_x, vert_x));
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
        #[cfg(feature = "profiler")]
        profile_scope!("steps_to");

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

// TODO: Make this be a BTreeMap or something that lets me search faster
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

pub fn stage1(panel: &Panel) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");
    let last = panel.intersections(0, 1).chain(panel.intersections(1, 0));
    // log::info!("{:?}", last.collect::<Vec<Point>>());
    last.map(manhattan_distance).min().unwrap()
}

pub fn stage2(panel: &Panel) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");

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
    profile_scope!("day3");
    let start = Instant::now();
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
    let panel = Panel::new();
    let panel = input
        .iter()
        .fold(panel, |p, v| p.construct_wire(v.to_vec()));
    log::debug!("Day 3 loading timer: {:?}", start.elapsed());

    // log::info!("{:?}", input);
    let start = Instant::now();
    let s1 = stage1(&panel);
    log::debug!("Stage 1 timer: {:?}", start.elapsed());
    log::info!("{:?}", s1);

    let start = Instant::now();
    let s2 = stage2(&panel);
    log::debug!("Stage 2 timer: {:?}", start.elapsed());
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
        let p = Panel::new();
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
        let p = Panel::new();
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
        let p = Panel::new();
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
