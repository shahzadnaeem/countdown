use anyhow::Result;
use clap::Parser;
use countdown::combs::{solutions2, Result as SolutionResult};
use countdown::defs::{all_ops, std_ops, Op, OpsType};

#[derive(Parser)]
#[command(version, about)]
struct Countdown {
    #[arg(short = 't')]
    target: i32,
    nums: Vec<i32>,
    #[arg(long = "op", action = clap::ArgAction::Append)]
    ops: Vec<Op>,
    #[arg(long, default_value_t = true)]
    std_ops: bool,
    #[arg(long, default_value_t = false)]
    all_ops: bool,
}

// Invocation: CLAP provides help
//
// countdown 50 25 75 100 4 1 -t 608
//

pub fn solve(target: i32, nums: &[i32], ops: &OpsType) {
    let solns = solutions2(&nums, target, ops);

    solns.0.iter().for_each(|s| {
        println!("{} = {}", s.0, s.1);
    });

    println!(
        "{} solutions to make {} from {:?} - {} checked",
        solns.0.len(),
        target,
        nums,
        solns.1
    );

    let mut deduped = Vec::<SolutionResult>::new();

    solns.0.clone().into_iter().for_each(|s| {
        if let Some(dup) = deduped.iter().find(|&dr| dr == &s) {
            println!("Duplicate: {} of {}", s.0, dup.0);
        } else {
            deduped.push(s);
        }
    });

    if deduped.len() != solns.0.len() {
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
}

fn validate_args(countdown: &mut Countdown) -> Result<bool> {
    if countdown.all_ops && countdown.ops.is_empty() {
        countdown.ops = all_ops();
    } else if countdown.std_ops && countdown.ops.is_empty() {
        countdown.ops = std_ops();
    } else {
        let mut ops = OpsType::new();

        countdown.ops.iter().for_each(|op| {
            if !ops.contains(op) {
                ops.push(op.clone())
            }
        });

        countdown.ops = ops;
    }

    Ok(true)
}

pub fn main() -> Result<()> {
    let mut countdown = Countdown::parse();

    validate_args(&mut countdown)?;

    solve(countdown.target, &countdown.nums, &countdown.ops);

    Ok(())
}
