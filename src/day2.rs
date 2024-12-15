use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Int     = i32;
type Report  = Vec<Int>;
type Reports = Vec<Report>;
  
const FILEPTH:&str = "./data/day2.txt";

pub fn day2() -> (Int,Int) {
    let reports:Reports = read_datafile(FILEPTH).unwrap();
    (part1(&reports), part2(&reports))
}

// Part 1
fn part1(reports:&Reports) -> Int {
    let mut ct = 0;
    for report in reports {
        let n = report.len();
        assert!(n >= 2);
        if check_report(&report) {
            ct += 1;
        }
    }
    ct
}

fn part2(reports:&Reports) -> Int {
    let mut ct = 0;
    for report in reports {
        if check_report(&report) {
            ct += 1;
        } else {
            for i in 0..report.len() {
                let mut adjusted_report = report.clone();
                adjusted_report.remove(i);
                if check_report(&adjusted_report) {
                    ct += 1;
                    break;
                }
            }
        }

    }
    ct
}

fn read_datafile(fpth:&str) -> io::Result<Reports> {
    let file:File = File::open(fpth)?;
    let reader:BufReader<File> = BufReader::new(file);
    let mut data:Reports = Vec::new();
    for line in reader.lines() {
        let line: String = line?;
        let mut report:Report = Vec::new();
        let res = line.split_whitespace();
        for val in res {
            report.push(val.parse().unwrap());
        }
        data.push(report);
    }
    Ok(data)
}

fn check_report(report:&Report) -> bool {
    const MAX_GAP:Int = 3;
    let mut is_mono = report[1] != report[0];
    let mut is_gradual = (report[0] - report[1]).abs() <= MAX_GAP;
    let pos_slope = report[1] > report[0];

    let n = report.len();
    let mut i = 2;
    while i < n && is_gradual && is_mono {
        let gap = report[i] - report[i-1];
        match pos_slope {
            true  => is_mono = gap > 0,
            false => is_mono = gap < 0
        }
        if gap == 0 {
            is_mono = false;
            break;
        }

        if !is_mono {
            break;
        }

        if gap.abs() > MAX_GAP {
            is_gradual = false;
            break;
        }
        i += 1;
    }


    is_mono && is_gradual
}


/// Testing
#[cfg(test)]
mod tests {
    use super::Reports;

    use super::part1;
    use super::part2;
    use super::Int;

    // Part 1 Test Case
    #[test]
   fn test_pt1() {
    let exp:Int = 2;
    let mut inp:Reports = Vec::new();
    inp.push(vec![7,6,4,2,1]);
    inp.push(vec![1,2,7,8,9]);
    inp.push(vec![9,7,6,2,1]);
    inp.push(vec![1,3,2,4,5]);
    inp.push(vec![8,6,4,4,1]);
    inp.push(vec![1,3,6,7,9]);
    let act = part1(&inp);

    assert_eq!(exp,act);
   }

    // Part 2 Test Case
    #[test]
    fn test_pt2() {
     let exp:Int = 4;
     let mut inp:Reports = Vec::new();
     inp.push(vec![7,6,4,2,1]);
     inp.push(vec![1,2,7,8,9]);
     inp.push(vec![9,7,6,2,1]);
     inp.push(vec![1,3,2,4,5]);
     inp.push(vec![8,6,4,4,1]);
     inp.push(vec![1,3,6,7,9]);
     let act = part2(&inp);

     assert_eq!(exp,act);
    }
}