mod day1;
fn main() {
    use std::time::Instant;
    use std::time::Duration;
    use crate::day1;
    
    // Day 1
    let now: Instant = Instant::now();
    { 
        let ans1:(i32, i32) = day1::day1(); 
        println!("Day1:");
        println!("-part1: {}",ans1.0);
        println!("-part2: {}",ans1.1);
    }
    let elapsed: Duration = now.elapsed();
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();
}
