use std::fs;
use radix_fmt::radix_5;
use std::fmt::Write;


fn snafu_to_base10(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate()
         .fold(0i64, |acc, (idx, ch)| {
             let value : i64 = match ch {
                 '=' => -2,
                 '-' => -1,
                  x => x.to_digit(5).unwrap() as i64,
             };
             acc + value * 5i64.pow(idx as u32)
         })
}

fn base10_to_snafu(value: &i64) -> String {
    let mut s = String::new();
    write!(&mut s, "{}", radix_5(*value)).unwrap();

    let mut digits_out: Vec<char> = Vec::new();
    let mut carryover : i8 = 0;
    for mut digit in s.chars().rev().map(|c| c.to_digit(5).unwrap() as i8) {
        digit += carryover;
        if digit > 2 {
            digit -= 5;
            carryover = 1;
        } else {
            carryover = 0;
        }
        digits_out.push("=-012".chars().nth((digit + 2) as usize).unwrap());
    }
    return digits_out.into_iter().rev().collect();
}

fn day24() {
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let mut total = 0;
    for line in file.lines() {
        total += snafu_to_base10(line);
    }
    println!("Total: {}, which in snafu is '{}'.", total, base10_to_snafu(&total)); //ans: 2-=102--02--=1-12=22
}

fn main() {
    day24();
}

#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(snafu_to_base10("1"), 1);
        assert_eq!(snafu_to_base10("1="), 3);
        assert_eq!(snafu_to_base10("2-"), 9);
        assert_eq!(snafu_to_base10("20"), 10);
        assert_eq!(snafu_to_base10("1=11-2"), 2022);
        assert_eq!(snafu_to_base10("1-0---0"), 12345);
        assert_eq!(snafu_to_base10("1121-1110-1=0"), 314159265);
    }
}