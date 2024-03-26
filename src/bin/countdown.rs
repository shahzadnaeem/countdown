use clap::Parser;
use expr::combs::{solutions2, Result};

#[derive(Parser)]
#[command(version, about)]
struct Countdown {
    #[arg(short = 't')]
    target: i32,
    nums: Vec<i32>,
}

// Invocation:
//
// countdown 608 50 25 75 100 4 1
//

pub fn solve(target: i32, nums: &[i32]) {
    let solns = solutions2(&nums, target);

    solns.0.iter().for_each(|s| {
        println!("{} = {}", s.0, s.1);
    });

    println!(
        "{} solutions to make {} from {:?} - {} checked\n",
        solns.0.len(),
        target,
        nums,
        solns.1
    );

    let mut deduped = Vec::<Result>::new();

    solns.0.clone().into_iter().for_each(|s| {
        if let Some(dup) = deduped.iter().find(|&dr| dr == &s) {
            println!("Duplicate: {} of {}", s.0, dup.0);
        } else {
            deduped.push(s);
        }
    });

    println!();

    deduped.iter().for_each(|s| {
        println!("{} = {}", s.0, s.1);
    });

    println!(
        "{} unique solutions to make {} from {:?}",
        deduped.len(),
        target,
        nums
    );
}

pub fn main() {
    let countdown = Countdown::parse();

    solve(countdown.target, &countdown.nums);
}