use anyhow::Result;
use clap::Parser;
use countdown::combs::{solutions2, Result as SolutionResult};
use countdown::defs::{all_ops, std_ops, Op, OpsType};

#[derive(Parser)]
#[command(version = "1.0", about = "Countdown numbers solver")]
struct Countdown {
    #[arg(short = 't', long = "target", help = "Target value")]
    target: i32,
    #[arg(help = "Allowed numbers, used once, can include duplicates")]
    nums: Vec<i32>,

    #[arg(long = "op", name = "OP", action = clap::ArgAction::Append)]
    ops: Vec<Op>,
    #[arg(long, default_value_t = true, help = "[add, mul, sub, div] -- default")]
    std_ops: bool,
    #[arg(long, default_value_t = false, help = "[add, mul, sub, div, exp]")]
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
    let mut first_dup = true;

    solns.0.clone().into_iter().for_each(|s| {
        if let Some(dup) = deduped.iter().find(|&dr| dr == &s) {
            if first_dup {
                println!();
                first_dup = false;
            }
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
            "{} unique solutions to make {} from {:?} - {} checked",
            deduped.len(),
            target,
            nums,
            solns.1
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
