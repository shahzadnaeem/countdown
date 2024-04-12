use anyhow::Result;
use clap::Parser;
use countdown::expr::{all_ops, std_ops, Op, OpsType};
use countdown::solver_v2::{dedup, solutions2};

const ABOUT: &str = r#"Countdown numbers solver

Examples:
  countdown 25 50 75 100 3 6 -t 952          # A famous real game
  countdown 1 2 3 4 --op add --op mul -t 32  # Can use fewer numbers and operators
  countdown 1 2 3 4 --all-ops -t 32          # Can use extra operators
"#;

#[derive(Parser)]
#[command(version = "1.0", about = ABOUT)]
struct Countdown {
    #[arg(short = 't', long = "target", help = "Target value")]
    target: i32,
    #[arg(help = "Allowed numbers, used once, can include duplicates")]
    nums: Vec<i32>,

    #[arg(long = "op", name = "OP", action = clap::ArgAction::Append)]
    ops: Vec<Op>,
    #[arg(long, default_value_t = true, help = "[add, mul, sub, div] -- default")]
    std_ops: bool,
    #[arg(long, default_value_t = false, help = "[add, mul, sub, div, exp, mod]")]
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

    let (deduped, dups_log) = dedup(&solns.0);

    if !dups_log.is_empty() {
        println!("\nDuplicates: {} found", dups_log.len());
        dups_log.iter().for_each(|s| println!("  {s}"));
    }

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
