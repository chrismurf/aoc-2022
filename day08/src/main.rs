// This implementation is awful and I stopped caring at some point.

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::izip;
use std::iter::zip;

type HeightMap = Vec<Vec<u8>>;

#[derive(Default, Debug)]
struct VisibilityMaps {
    left : HeightMap,
    right : HeightMap,
    top : HeightMap,
    bottom : HeightMap,
}

impl VisibilityMaps {
    fn from_height_map(height: &HeightMap) -> Self {
        let mut left : HeightMap = Vec::new();
        let mut right : HeightMap = Vec::new();
        let mut top : HeightMap = Vec::new();
        let mut bottom : HeightMap = Vec::new();

        // First compute left / right / top visibility
        let mut visible_from_top : Vec<u8> = vec![0; height[0].len()];
        for row in height.iter() {
            let mut visible_from_left : u8 = 0;
            left.push(
                row.iter().map(|c| {
                    if c >= &visible_from_left {
                        let previous = visible_from_left;
                        visible_from_left = c + 1;
                        return previous;
                    } else {
                        return visible_from_left
                    }
                }).collect()
            );

            let mut visible_from_right : u8 = 0;
            let row_reversed = row.iter().rev().map(|x| *x).collect::<Vec<u8>>();
            let zz : Vec<u8> = row_reversed.iter().map(|c| {
                    if c >= &visible_from_right {
                        let previous = visible_from_right;
                        visible_from_right = c + 1;
                        return previous;
                    } else {
                        return visible_from_right
                    }
                }).collect();
            right.push(zz.iter().rev().map(|x| *x).collect());

            top.push(visible_from_top.clone());
            for (r, v) in zip(row.iter(), visible_from_top.iter_mut()) {
                if r >= v {
                    *v = r + 1;
                }
            }
        }

        // Now compute bottom visibility
        let mut visible_from_bottom : Vec<u8> = vec![0; height[0].len()];
        for row in height.iter().rev() {
            bottom.push(visible_from_bottom.clone());
            for (r, v) in zip(row.iter(), visible_from_bottom.iter_mut()) {
                if r >= v {
                    *v = r + 1;
                }
            }
        }
        bottom.reverse();

        Self {left, right, top, bottom}
    }

    fn overall_visibility(&self) -> HeightMap {
        izip!(&self.left, &self.right, &self.top, &self.bottom).map(|(l, r, t, b)| {
            izip!(l, r, t, b).map(|(lv, rv, tv, bv)| {
                *[*lv, *rv, *tv, *bv].iter().min().unwrap()
            }).collect()
        }).collect()
    }

    fn count_visible(&self, height: &HeightMap) -> usize {
        let visibility = self.overall_visibility();
        zip(height, visibility).map(|(h, v)| {
            zip(h, v).filter(
                |(hv, vv)| { *hv >= vv }
            ).count()
        }).sum()
    }
}

fn parse_height_map(filename: &str) -> HeightMap {
    let file = File::open(filename).expect(format!("File {} not readable", filename).as_str());
    let height_map : HeightMap = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let first_line_length = height_map[0].len();
    assert!(height_map.iter().all(|x| x.len() == first_line_length));
    height_map
}

fn get_scenic_scores(height: &HeightMap) -> Vec<Vec<u32>> {
    let h = height.len();
    let w = height[0].len();
    let mut scores : Vec<Vec<u32>> = vec![vec![0; w]; h];
    for vr in 0..h {
        for vc in 0..w {
            let value = height[vr][vc];
            let mut left = 0;
            for cc in (0..vc).rev() {
                left += 1;
                if height[vr][cc] >= value { break; }
            }
            let mut right = 0;
            for cc in vc+1..w {
                right += 1;
                if height[vr][cc] >= value { break; }
            }
            let mut top = 0;
            for cr in (0..vr).rev() {
                top += 1;
                if height[cr][vc] >= value { break; }
            }
            let mut bottom = 0;
            for cr in vr+1..w {
                bottom += 1;
                if height[cr][vc] >= value { break; }
            }

            scores[vr][vc] = left * right * top * bottom;
        }
    }
    scores
}

fn day08() {
    let height = parse_height_map("input.txt");
    let visibility_maps = VisibilityMaps::from_height_map(&height);

    println!("Part 1: {:?}", visibility_maps.count_visible(&height));
    println!("Part 2: {:?}", get_scenic_scores(&height).iter().map(|x| x.iter().max().unwrap()).max().unwrap());
}

fn main() -> io::Result<()> {
    day08();
    Ok(())
}
