mod day1;
mod day2;

fn main() {
    use std::time::Instant;
    use std::time::Duration;
    use crate::day1;
    
    // Day 1
    let mut now: Instant = Instant::now();
    { 
        let ans1:(i32, i32) = day1::day1(); 
        println!("Day1:");
        println!("-part1: {}",ans1.0);
        println!("-part2: {}",ans1.1);
    }
    let mut elapsed: Duration = now.elapsed();
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    // Day 2
    now = Instant::now();
    {
        let ans2 = day2::day2();
        println!("Day2:");
        println!("-part1: {}",ans2.0);
        println!("-part2: {}",ans2.1);
    }
    elapsed = now.elapsed();
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();
}
