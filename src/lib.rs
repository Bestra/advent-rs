use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

pub fn read_input(year: &str, day: &str) -> Vec<String> {
    let path = format!("./inputs/{}/{}.txt", year, day);
    if !Path::new(&path).exists() {
        eprintln!("{} does not exist", path);
        exit(1);
    }
    let f = File::open(path).unwrap();
    let buf = BufReader::new(f);
    buf.lines().map(|l| l.unwrap()).collect()
}

pub mod year_2018 {
    pub mod day_1 {
        pub fn part_1(input: Vec<String>) -> i32 {
            let line = &input[0];
            let mut pos: i32 = 0;
            for c in line.chars() {
                match c {
                    '(' => pos += 1,
                    ')' => pos -= 1,
                    _ => ()
                }
            }

            return pos;
        }

        #[cfg(test)]
        mod tests {
            use super::*;
        
            #[test]
            fn pt_1() {
                assert_eq!(0, part_1(vec!["(())".to_owned()]))
            }
        }
    }
}