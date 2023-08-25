use crate::rookrustpad::service::aoc::AocFunctionResult;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::IntoIterator;
use std::error::Error;

use lazy_regex::regex;

 #[derive(Debug, Clone)]
pub enum Expression {
    Old,
    Number(u64),
    Multiply(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>)
}

impl Expression {
    pub fn eval(&self, initial: u64) -> u64 {
        match self {
            Expression::Old =>
                initial,
            Expression::Number(n) =>
                *n,
            Expression::Multiply(left, right) =>
                left.eval(initial) * right.eval(initial),
            Expression::Add(left, right) =>
                left.eval(initial) + right.eval(initial)
        }
    }
}

#[derive(Debug)]
struct MonkeyInfo {
    id: u32,
    items: Vec<u64>,
    expression: Expression,
    test: u64,
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

    pub fn get_new_monkey_id(&self, item: u64) -> u32 {
        if item % self.test == 0 {
            self.on_true
        }
        else {
            self.on_false
        }
    }

    pub fn test_and_get_throws(&mut self) -> Vec<(u32, u64)> {
        let items = core::mem::replace(&mut self.items, Vec::new());
        let mut moves: Vec<(u32, u64)> = Vec::new();

        for item in items {
            moves.push((self.get_new_monkey_id(item), item));
        }

        moves
    }

}

struct MonkeyInfoParser {
}

impl MonkeyInfoParser {

    pub fn read_all_from_file(input_path: &String) -> Result<Vec<MonkeyInfo>, Box<dyn Error>>
    {
        let file = File::open(input_path)?;
        let mut reader = BufReader::new(file);
        MonkeyInfoParser::read_all(&mut reader)
    }

    pub fn read_all<R: BufRead>(reader: &mut R) -> Result<Vec<MonkeyInfo>, Box<dyn Error>>
    {
        let mut monkey_infos: Vec<MonkeyInfo> = Vec::new();

        loop {
            match MonkeyInfoParser::read(reader)? {
                None => {
                    return Ok(monkey_infos);
                },
                Some(m_info) => {
                    monkey_infos.push(m_info);
                }
            }
        }
    }

    pub fn read<R: BufRead>(reader: &mut R) -> Result<Option<MonkeyInfo>, Box<dyn Error>>
    {
        let mut line_buf = String::new();

        if !MonkeyInfoParser::read_until_non_blank(reader, &mut line_buf)? {
            // At end of stream.
            return Ok(None);
        }

        let id = MonkeyInfoParser::read_monkey_line(&mut line_buf)?;

        MonkeyInfoParser::read_next_line(reader, &mut line_buf, "starting items")?;
        let items = MonkeyInfoParser::read_starting_items(&line_buf)?;

        MonkeyInfoParser::read_next_line(reader, &mut line_buf, "operation")?;
        let expression = MonkeyInfoParser::read_operation(&line_buf)?;

        MonkeyInfoParser::read_next_line(reader, &mut line_buf, "test")?;
        let test = MonkeyInfoParser::read_test(&line_buf)?;

        MonkeyInfoParser::read_next_line(reader, &mut line_buf, "test handler")?;
        let h1 = MonkeyInfoParser::read_test_handler(&line_buf)?;

        MonkeyInfoParser::read_next_line(reader, &mut line_buf, "test handler")?;
        let h2 = MonkeyInfoParser::read_test_handler(&line_buf)?;

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

    fn read_starting_items(line: &String) -> Result<Vec<u64>, Box<dyn Error>>
    {
        let starting_items_re = regex!(r"^\s*Starting items:\s+([\d,\s]+)\s*");
        let cap = starting_items_re.captures(&line).ok_or(format!("Invalid starting items: {}", line))?;

        let mut items: Vec<u64> = Vec::new();

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

    fn read_test(line: &String) -> Result<u64, Box<dyn Error>> {
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
    monkees: HashMap<u32, MonkeyInfo>,
    ids: Vec<u32>
}

impl Monkees {

    pub fn new<I>(monkees: I) -> Monkees
        where I: IntoIterator<Item=MonkeyInfo>
    {
        let mut m = Monkees {
            monkees: HashMap::new(),
            ids: Vec::new()
        };

        for monkey in monkees {
            let id = monkey.id;
            m.monkees.insert(id, monkey);
            m.ids.push(id);
        }

        m.ids.sort();

        m
    }

    pub fn get_modulus(&self) -> u64 {
        self.monkees.values().fold(1, |a, b| { a * b.test })
    }

    pub fn run_rounds<F>(&mut self, rounds: u32, manage_worry: F)
        where F: Fn(&mut MonkeyInfo) -> ()
    {
        for _ in 1..=rounds {
            self.run_round(&manage_worry);
        }
    }

    pub fn run_round<F>(&mut self, manage_worry: F)
        where F: Fn(&mut MonkeyInfo) -> ()
    {
        for m_id in &self.ids {
            let monkey = self.monkees.get_mut(m_id).unwrap();
            monkey.inspect_items();
            manage_worry(monkey);

            let throws = monkey.test_and_get_throws();
            for (new_mid, item) in throws {
                self.monkees.get_mut(&new_mid).unwrap().items.push(item);
            }
        }
    }

    pub fn get_monkey_business(&self) -> u64 {
        let mut counts: Vec<u32> = self.monkees.iter().map(|(_, m)| { m.inspection_count }).collect();
        counts.sort();

        let monkey_business = u64::from(counts[counts.len() - 1]) * u64::from(counts[counts.len() - 2]);
        monkey_business
    }

    pub fn load(input_path: String) -> Result<Monkees, Box<dyn Error>> {
        let all_monkees = MonkeyInfoParser::read_all_from_file(&input_path)?;
        Ok(Monkees::new(all_monkees))
    }
}

pub fn part1(input_path: String, _log: &mut String) -> AocFunctionResult {
    let mut monkees = Monkees::load(input_path)?;

    monkees.run_rounds(20, |m| {
        for i in 0..m.items.len() {
            m.items[i] = m.items[i] / 3;
        }
    });

    let monkey_business = monkees.get_monkey_business();
    Ok(format!("{}", monkey_business))
}


pub fn part2(input_path: String, _log: &mut String) -> AocFunctionResult {
    let mut monkees = Monkees::load(input_path)?;
    let modulus = monkees.get_modulus();

    monkees.run_rounds(10000, |m| {
        for i in 0..m.items.len() {
            m.items[i] = m.items[i] % modulus;
        }
    });

    let monkey_business = monkees.get_monkey_business();
    Ok(format!("{}", monkey_business))
}