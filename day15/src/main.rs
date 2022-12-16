use std::cmp::{min, max};
use std::{fs, io};
use std::collections::{VecDeque, HashSet};
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone)]
struct Span { min: i32, max: i32 }

impl Span {
    fn overlaps(&self, other: &Span) -> bool {
        !(self.min > other.max || self.max < other.min)
    }

    fn count(&self) -> usize {
        (self.max - self.min + 1) as usize
    }
}

#[derive(Debug, Clone)]
struct SpanSet {
    spans: VecDeque<Span>
}

impl SpanSet {
    fn new() -> Self { Self { spans: VecDeque::new() } }


    fn from_spans(mut spans: Vec<Span>) -> Self {
        spans.sort_by(|a, b| a.min.cmp(&b.min) );

        let mut out = VecDeque::new();

        for span in spans {
            if out.is_empty() {
                out.push_back(span);
                continue;
            }

            let mut back = out.back_mut().unwrap();

            if span.overlaps(back) {
                back.max = max(span.max, back.max);
            } else {
                out.push_back(span);
            }
        }

        SpanSet { spans: out }
    }

    fn from_regions_and_row(regions: &Vec<SearchedRegion>, y: i32) -> Self {
        SpanSet::from_spans(
            regions.iter().filter_map(|x| x.bounds_for_row(y)).collect()
        )
    }

    fn count(&self) -> usize {
        self.spans.iter().map(|x| x.count()).sum()
    }

    fn intersection(&self, bounds: Span) -> SpanSet {
        let mut intersection = SpanSet::new();

        for span in &self.spans {
            if bounds.overlaps(span) {
                intersection.spans.push_back(
                    Span {min : max(span.min, bounds.min), max: min(span.max, bounds.max) }
                );
            }
        }

        intersection
    }
}

#[derive(Debug, Clone)]
struct SearchedRegion {
    /// A region that has no *unidentified* beacons, defined by a center and L1 radius
    x0: i32,
    y0: i32,
    distance: u32,
}

impl SearchedRegion {
    fn bounds_for_row(&self, y: i32) -> Option<Span> {
        // Searched columns have a distance <= `distance` from x0, y0.
        let dx : i32 = self.distance as i32 - (self.y0 - y).abs() as i32;
        // If no searched columns in this row, return empty set
        if dx < 0 { return None }
        // Otherwise return bounds on that row
        Some(Span {min: self.x0 - dx, max: self.x0 + dx})
    }
}


fn parse_row(text: &str) -> (SearchedRegion, (i32, i32)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Sensor at x=(?P<x0>-?\d+), y=(?P<y0>-?\d+): closest beacon is at x=(?P<xb>-?\d+), y=(?P<yb>-?\d+)").unwrap();
    }
    let caps = RE.captures(text).unwrap();
    let x0 : i32 = caps.name("x0").unwrap().as_str().parse().unwrap();
    let y0 : i32 = caps.name("y0").unwrap().as_str().parse().unwrap();
    let xb : i32 = caps.name("xb").unwrap().as_str().parse().unwrap();
    let yb : i32 = caps.name("yb").unwrap().as_str().parse().unwrap();

    let distance = ((xb-x0).abs() + (yb-y0).abs()) as u32;

    (SearchedRegion { x0, y0, distance }, (xb, yb))
}

pub fn day15() -> Result<(), Error> {

    let file = fs::File::open("input.txt").expect("File 'input.txt' not readable.");
    let reader = io::BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    let mut regions = Vec::new();
    let mut beacons = HashSet::new();

    // First, parse all the Searched Regions
    for row in reader {
        let (region, beacon) = parse_row(&row);
        regions.push(region);
        beacons.insert(beacon);
    }

    let spans = SpanSet::from_regions_and_row(&regions, 2000000);
    let count = spans.count() - beacons.iter().filter(|x| x.1 == 2000000).count();
    println!("Part 1: {}", count);

    for y in 0 ..= 4000000 {
        let spans = SpanSet::from_regions_and_row(&regions, y);
        let intersection = spans.intersection(Span {min: 0, max: 4000000} );
        if intersection.count() != 4000001 {
            let x = intersection.spans[0].max + 1;
            let frequency = 4000000*(x as u64) + y as u64;
            println!("Part 2: {},{} => {}", x, y, frequency);
            break;
        }
    }

    Ok(())
}

pub fn main() -> Result<(), Error> {
    day15()
}
