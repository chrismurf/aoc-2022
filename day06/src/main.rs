use std::{io, fs};
use std::collections::HashSet;

fn detect_unique_sequence(stream: &str, length: usize) -> i32 {
    for (i, elts) in stream.as_bytes().windows(length).enumerate() {
        let set : HashSet<u8> = elts.iter().copied().collect();
        if set.len() == length {
            return (i + length).try_into().unwrap();
        }
    }
    return -1;
}

fn day06() {
    let stream = fs::read_to_string("input.txt").unwrap();
    let part1 = detect_unique_sequence(&stream, 4);
    let part2 = detect_unique_sequence(&stream, 14);

    println!("{}, {}", part1, part2);
}

fn main() -> io::Result<()> {
    day06();
    Ok(())
}
#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(7, detect_unique_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, detect_unique_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, detect_unique_sequence("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, detect_unique_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, detect_unique_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test_two() {
        assert_eq!(19, detect_unique_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, detect_unique_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, detect_unique_sequence("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, detect_unique_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, detect_unique_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}