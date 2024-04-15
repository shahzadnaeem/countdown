use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::io::{self, BufRead};

use countdown::expr::*;

#[derive(pest_derive::Parser)]
#[grammar = "expr.pest"]
pub struct ExprParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left) | Op::infix(exp, Left))
            .op(Op::prefix(unary_minus))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Val(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Sub,
                Rule::multiply => Op::Mul,
                Rule::divide => Op::Div,
                Rule::modulo => Op::Mod,
                Rule::exp => Op::Exp,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::new_expr(op, lhs, rhs)
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::new_expr(Op::Sub, Expr::new_val(0), rhs),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match ExprParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Parsed: {:#?}", expr);

                let result = eval_freely(&expr);

                println!("Evaluated: {:?}", result);
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}
