use std::fs::File;
use std::io::{prelude::*, BufReader};

fn do_mixing(org_list: &Vec<isize>, iterations: u32) -> Vec<isize> {
    // vector of current locations, indexed by original location
    let n = org_list.len();
    let mut org_to_cur: Vec<usize> = (0..n).collect();
    let mut cur_to_org = org_to_cur.clone();

    for _ in 0..iterations {
        for org_idx in 0..n {
            let cur_idx = org_to_cur[org_idx];
            // If we're going to wrap around, we are better off going the other direction.
            // This insanity figures out if we will wrap, and modulo's us back down to a positive
            // or negative offset that does NOT wrap.  Note that % is NOT MODULO in Rust, it is
            // remainder, which can be negative.  Learn something new every day.
            let rotation = (cur_idx as isize + org_list[org_idx]).rem_euclid(n as isize - 1) - cur_idx as isize;


            // We are going to swap two at a time, because when I tried to just "jump to the result"
            // it went really, really badly.
            let sign = rotation.signum() as isize;
            for i in 0..rotation.abs() as isize {
                // If rotation is positive, we want to swap with the next cell
                // swap(i, i+signum)
                let idx_a = (cur_idx as isize + (i*sign)) as usize;
                let idx_b = (cur_idx as isize + ((i+1)*sign)) as usize;

                (cur_to_org[idx_a], cur_to_org[idx_b]) = (cur_to_org[idx_b], cur_to_org[idx_a]);
                org_to_cur[cur_to_org[idx_a]] = idx_a;
                org_to_cur[cur_to_org[idx_b]] = idx_b;
            }
        }
    }

    // Now return the list as it actually should be
    cur_to_org.iter()
        .map(|idx| org_list[*idx])
        .collect()
}

const DECRYPTION_KEY :isize = 811589153;

fn get_coords(v: &Vec<isize>) -> isize {
    let mut sum: isize = 0;
    let zero_idx = v.iter().position(|x| *x == 0).unwrap();
    for offset in [1000usize, 2000, 3000] {
        let idx = (zero_idx + offset) % v.len();
        sum += v[idx];
    }
    sum
}

pub fn day20() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let data : Vec<isize> = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()) // Get Strings instead of Result
        .filter_map(|line| line.parse::<isize>().ok())
        .collect();

    // Part A
    let out = do_mixing(&data, 1);
    let sum = get_coords(&out);
    println!("Part 1 total is {}", sum);

    // Part B
    let data_b = data.iter().map(|x| *x * DECRYPTION_KEY).collect::<Vec<isize>>();
    let out_b = do_mixing(&data_b, 10);
    let sum_b = get_coords(&out_b);
    println!("Part 2 total is {}", sum_b);
}

pub fn main() {
    day20()
}

#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
        let data = vec![1isize, 2, -3, 3, -2, 0, 4];
        let out = do_mixing(&data, 1);
        let sum = get_coords(&out);
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_two() {
        let data = vec![1isize, 2, -3, 3, -2, 0, 4].iter().map(|x| *x * DECRYPTION_KEY).collect();
        let out = do_mixing(&data, 10);
        let sum = get_coords(&out);
        assert_eq!(sum, 1623178306);
    }
}