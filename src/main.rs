mod day1;
fn main() {
    use std::time::Instant;
    use crate::day1;
    let now = Instant::now();
    {
        day1::day1();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
