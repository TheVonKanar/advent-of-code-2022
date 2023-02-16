use std::ops::Div;

use super::{get_puzzle, Info, Solution, Solver};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn init_monkeys(puzzle: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for group in puzzle.split("\n\n") {
        let mut lines = group.lines();
        lines.next();

        // Starting Items.
        let items: Vec<usize> = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        // Operation.
        let operation = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Operation: ")
            .unwrap()
            .to_string();

        // Test.
        let test = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // If True.
        let if_true = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // If False.
        let if_false = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // Add monkey to list.
        monkeys.push(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
        });
    }

    monkeys
}

fn parse_monkey_items<F>(monkeys: &mut Vec<Monkey>, post_op: F)
where
    F: Fn(usize) -> usize,
{
    for i in 0..monkeys.len() {
        while let Some(old) = monkeys[i].items.pop() {
            let op_split: Vec<&str> = monkeys[i].operation.split(' ').collect();
            let a = match op_split[2].parse::<usize>() {
                Ok(x) => x,
                Err(_) => old,
            };

            let b = match op_split[4].parse::<usize>() {
                Ok(x) => x,
                Err(_) => old,
            };

            let new = post_op(match op_split[3] {
                "+" => a + b,
                "*" => a * b,
                &_ => todo!(),
            });

            if new % monkeys[i].test == 0 {
                let if_true = monkeys[i].if_true;
                monkeys[if_true].items.push(new);
            } else {
                let if_false = monkeys[i].if_false;
                monkeys[if_false].items.push(new);
            }

            monkeys[i].inspections += 1;
        }
    }
}

pub(super) fn resolve(solver: &Solver, mut solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut monkeys_1: Vec<Monkey> = init_monkeys(puzzle);
    let mut monkeys_2: Vec<Monkey> = monkeys_1.clone();

    // Part 1.
    for _ in 0..20 {
        parse_monkey_items(&mut monkeys_1, |x| x.div(3));
    }

    monkeys_1.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
    solution.0 = (monkeys_1[0].inspections * monkeys_1[1].inspections).to_string();

    // Part 2: 10k rounds and no division by 3 of new worry level.
    // This means the items will eventually be way too big for usize or even u128
    // The solution is to find a common divider between all monkey "test" divisions
    // We do this by multiplying (product) every test value together
    let common_div: usize = monkeys_2.iter().map(|monkey| monkey.test).product();
    for _ in 0..10000 {
        parse_monkey_items(&mut monkeys_2, |x| x % common_div);
    }

    monkeys_2.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
    solution.1 = (monkeys_2[0].inspections * monkeys_2[1].inspections).to_string();
}

#[test]
fn test_11() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(11, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "55458");
    assert_eq!(solution.1, "14508081294");
}
