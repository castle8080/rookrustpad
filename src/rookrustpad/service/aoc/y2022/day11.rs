use crate::rookrustpad::service::aoc::AocFunctionResult;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::IntoIterator;
use std::error::Error;

use lazy_regex::regex;

/*

Monkey 0:
  Starting items: 75, 75, 98, 97, 79, 97, 64
  Operation: new = old * 13
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 7

 */


 #[derive(Debug)]
enum Expression {
    Old,
    Number(u128),
    Multiply(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>)
}

impl Expression {
    pub fn eval(&self, old: u128) -> u128 {
        match &self {
            &Expression::Old => old,
            &Expression::Number(n) => *n,
            &Expression::Multiply(left, right) => {
                left.eval(old) * right.eval(old)
            },
            &Expression::Add(left, right) => left.eval(old) + right.eval(old)
        }
    }
}

#[derive(Debug)]
struct MonkeyInfo {
    id: u32,
    items: Vec<u128>,
    expression: Expression,
    test: u128,
    on_true: u32,
    on_false: u32,
    inspection_count: u32
}

impl MonkeyInfo {

    pub fn inspect_items(&mut self) {
        // Change worry level using expression.
        for item in &mut self.items {
            *item = self.expression.eval(*item);
        }
        self.inspection_count = self.inspection_count + u32::try_from(self.items.len()).unwrap();
    }

    pub fn reduce_worry(&mut self) {
        for item in &mut self.items {
            *item = *item / 3;
        }
    }

    pub fn test_and_get_throws(&mut self) -> Vec<(u32, u128)> {
        let moves: Vec<(u32, u128)> = self.items.iter().map(|item| {
            let new_mid =
                if (*item % self.test) == 0 {
                    self.on_true
                }
                else {
                    self.on_false
                };
            (new_mid, *item)
        }).collect();

        self.items = Vec::new();

        moves
    }

    pub fn read_all_from_file(input_path: &String) -> Result<Vec<MonkeyInfo>, Box<dyn Error>> {
        let file = File::open(input_path)?;
        let mut reader = BufReader::new(file);
        MonkeyInfo::read_all(&mut reader)
    }

    pub fn read_all<R: BufRead>(reader: &mut R) -> Result<Vec<MonkeyInfo>, Box<dyn Error>> {
        let mut monkey_infos: Vec<MonkeyInfo> = Vec::new();

        loop {
            match MonkeyInfo::read(reader)? {
                None => {
                    return Ok(monkey_infos);
                },
                Some(m_info) => {
                    monkey_infos.push(m_info);
                }
            }
        }
    }

    pub fn read<R: BufRead>(reader: &mut R) -> Result<Option<MonkeyInfo>, Box<dyn Error>> {
        let mut line_buf = String::new();

        if !MonkeyInfo::read_until_non_blank(reader, &mut line_buf)? {
            // At end of stream.
            return Ok(None);
        }

        let id = MonkeyInfo::read_monkey_line(&mut line_buf)?;

        MonkeyInfo::read_next_line(reader, &mut line_buf, "starting items")?;
        let items = MonkeyInfo::read_starting_items(&line_buf)?;

        MonkeyInfo::read_next_line(reader, &mut line_buf, "operation")?;
        let expression = MonkeyInfo::read_operation(&line_buf)?;

        MonkeyInfo::read_next_line(reader, &mut line_buf, "test")?;
        let test = MonkeyInfo::read_test(&line_buf)?;

        MonkeyInfo::read_next_line(reader, &mut line_buf, "test handler")?;
        let h1 = MonkeyInfo::read_test_handler(&line_buf)?;

        MonkeyInfo::read_next_line(reader, &mut line_buf, "test handler")?;
        let h2 = MonkeyInfo::read_test_handler(&line_buf)?;

        if h1.0 == h2.0 {
            return Err(format!("Both test handlers have the same condition.").into());
        }

        let (on_true, on_false) =
            if h1.0 {
                (h1.1, h2.1)
            }
            else {
                (h2.1, h1.1)
            };

        Ok(Some(MonkeyInfo { id, items, expression, test, on_true, on_false, inspection_count: 0 }))
    }

    fn read_next_line<R: BufRead>(reader: &mut R, line_buf: &mut String, operation: &str) -> Result<(), Box<dyn Error>> {
        line_buf.clear();
        if reader.read_line(line_buf)? == 0 {
            Err(format!("End of file trying to read: {}", operation).into())
        }
        else {
            Ok(())
        }
    }

    fn read_until_non_blank<R: BufRead>(reader: &mut R, line_buf: &mut String) -> Result<bool, Box<dyn Error>> {
        // Eat up blank lines;
        let ws_re = regex!(r"^\s*$");
        loop {
            line_buf.clear();
            let size_read = reader.read_line(line_buf)?;
            if size_read == 0 {
                return Ok(false);
            }
            if !ws_re.is_match(&line_buf) {
                return Ok(true)
            }
        }
    }

    fn read_monkey_line(line: &String) -> Result<u32, Box<dyn Error>> {
        let monkey_re = regex!(r"^\s*Monkey\s+(\d+):");
        let cap = monkey_re.captures(&line).ok_or(format!("Invalid initial line: {}", line))?;
        Ok(cap.get(1).unwrap().as_str().parse::<u32>()?)
    }

    fn read_starting_items(line: &String) -> Result<Vec<u128>, Box<dyn Error>> {
        let starting_items_re = regex!(r"^\s*Starting items:\s+([\d,\s]+)\s*");
        let cap = starting_items_re.captures(&line).ok_or(format!("Invalid starting items: {}", line))?;

        let mut items: Vec<u128> = Vec::new();

        for t in cap.get(1).unwrap().as_str().split(",") {
            let s = t.trim();
            items.push(s.parse()?);
        }

        Ok(items)
    }

    fn read_operation(line: &String) -> Result<Expression, Box<dyn Error>> {
        let operation_re = regex!(r"^\s*Operation: new = (old|\d+) ([\+\*]) (old|\d+)");
        let cap = operation_re.captures(&line).ok_or(format!("Invalid operation: {}", line))?;

        let left_op_str = cap.get(1).unwrap().as_str();
        let left_op = 
            if left_op_str == "old" {
                Expression::Old
            }
            else {
                Expression::Number(left_op_str.parse()?)
            };
        
        let right_op_str = cap.get(3).unwrap().as_str();
        let right_op = 
            if right_op_str == "old" {
                Expression::Old
            }
            else {
                Expression::Number(right_op_str.parse()?)
            };

        let op = cap.get(2).unwrap().as_str();
        if op == "+" {
            Ok(Expression::Add(Box::new(left_op), Box::new(right_op)))
        }
        else if op == "*" {
            Ok(Expression::Multiply(Box::new(left_op), Box::new(right_op)))
        }
        else {
            Err(format!("Invalid operation: {}", op).into())
        }
    }

    fn read_test(line: &String) -> Result<u128, Box<dyn Error>> {
        let test_re = regex!(r"^\s*Test: divisible by (\d+)");
        let cap = test_re.captures(line).ok_or(format!("Invalid test: {}", line))?;

        Ok(cap.get(1).unwrap().as_str().parse()?)
    }

    fn read_test_handler(line: &String) -> Result<(bool, u32), Box<dyn Error>> {
        let condition_re = regex!(r"^\s*If (true|false): throw to monkey (\d+)");
        let cap = condition_re.captures(line).ok_or(format!("Invalid test handler: {}", line))?;

        let condition = cap.get(1).unwrap().as_str().parse::<bool>()?;
        let to_monkey = cap.get(2).unwrap().as_str().parse::<u32>()?;

        Ok((condition, to_monkey))
    }
}

struct Monkees {
    monkees: HashMap<u32, MonkeyInfo>
}

impl Monkees {

    pub fn new() -> Monkees {
        Monkees {
            monkees: HashMap::new()
        }
    }

    pub fn add(&mut self, monkey: MonkeyInfo) {
        self.monkees.insert(monkey.id, monkey);
    }

    pub fn add_all<I: IntoIterator<Item=MonkeyInfo>>(&mut self, monkees: I) {
        for monkey in monkees {
            self.add(monkey);
        }
    }

    pub fn get_ids(&mut self) -> Vec<u32> {
        let mut m_ids: Vec<u32> = Vec::new();
        for k in self.monkees.keys() {
            m_ids.push(*k);
        }
        m_ids.sort();
        m_ids
    }

    pub fn run_round<F: Fn(&mut MonkeyInfo) -> ()>(&mut self, monkey_maint_f: F) {
        for m_id in self.get_ids() {
            let monkey = self.monkees.get_mut(&m_id).unwrap();
            monkey.inspect_items();
            monkey_maint_f(monkey);
            //monkey.reduce_worry();

            let throws = monkey.test_and_get_throws();
            for (new_mid, item) in throws {
                self.monkees.get_mut(&new_mid).unwrap().items.push(item);
            }
        }
    }
}

fn run_part<F: Fn(&mut MonkeyInfo) -> ()>(input_path: String, _log: &mut String, monkey_maint_f: F) -> AocFunctionResult {
    let mut monkees = Monkees::new();
    let all_monkees = MonkeyInfo::read_all_from_file(&input_path)?;
    monkees.add_all(all_monkees);

    for round in 1..=20 {
        println!("On round: {}", round);
        monkees.run_round(&monkey_maint_f);
    }

    let mut counts: Vec<u32> = monkees.monkees.iter().map(|(_, m)| { m.inspection_count }).collect();
    counts.sort();

    let monkey_business = counts[counts.len() - 1] * counts[counts.len() - 2];

    Ok(format!("{}", monkey_business))
}

pub fn part1(input_path: String, _log: &mut String) -> AocFunctionResult {
    run_part(input_path, _log, |m| { m.reduce_worry() })
}

pub fn part2(input_path: String, _log: &mut String) -> AocFunctionResult {
    run_part(input_path, _log, |m| { })
}