use crate::rookrustpad::service::aoc::AocFunctionResult;

use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, BufRead};
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
    Number(u32),
    Multiply(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
struct MonkeyInfo {
    id: u32,
    items: Vec<u32>,
    expression: Expression,
    test: i32,
    on_true: u32,
    on_false: u32
}

impl MonkeyInfo {

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
            if (h1.0) {
                (h1.1, h2.1)
            }
            else {
                (h2.1, h1.1)
            };

        Ok(Some(MonkeyInfo { id, items, expression, test, on_true, on_false }))
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

    fn read_starting_items(line: &String) -> Result<Vec<u32>, Box<dyn Error>> {
        let starting_items_re = regex!(r"^\s*Starting items:\s+([\d,\s]+)\s*");
        let cap = starting_items_re.captures(&line).ok_or(format!("Invalid starting items: {}", line))?;

        let mut items: Vec<u32> = Vec::new();

        for t in cap.get(1).unwrap().as_str().split(",") {
            let s = t.trim();
            items.push(s.parse()?);
        }

        Ok(items)
    }

    fn read_operation(line: &String) -> Result<Expression, Box<dyn Error>> {
        let operation_re = regex!(r"^\s*Operation: new = (old|[\d+]) ([\+\*]) (old|[\d+])");
        let cap = operation_re.captures(&line).ok_or(format!("Invalid operation: {}", line))?;

        let left_op_str = cap.get(1).unwrap().as_str();
        let left_op = 
            if left_op_str == "old" {
                Expression::Old
            }
            else {
                Expression::Number(left_op_str.parse::<u32>()?)
            };
        
        let right_op_str = cap.get(3).unwrap().as_str();
        let right_op = 
            if right_op_str == "old" {
                Expression::Old
            }
            else {
                Expression::Number(right_op_str.parse::<u32>()?)
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

    fn read_test(line: &String) -> Result<i32, Box<dyn Error>> {
        let test_re = regex!(r"^\s*Test: divisible by (\d+)");
        let cap = test_re.captures(line).ok_or(format!("Invalid test: {}", line))?;

        Ok(cap.get(1).unwrap().as_str().parse::<i32>()?)
    }

    fn read_test_handler(line: &String) -> Result<(bool, u32), Box<dyn Error>> {
        let condition_re = regex!(r"^\s*If (true|false): throw to monkey (\d+)");
        let cap = condition_re.captures(line).ok_or(format!("Invalid test handler: {}", line))?;

        let condition = cap.get(1).unwrap().as_str().parse::<bool>()?;
        let to_monkey = cap.get(2).unwrap().as_str().parse::<u32>()?;

        Ok((condition, to_monkey))
    }
}

pub fn part1(input_path: String, log: &mut String) -> AocFunctionResult {
    let monkees = MonkeyInfo::read_all_from_file(&input_path)?;

    for m in monkees {
        writeln!(log, " * {:?}", m)?;
    }

    Ok(String::from("The answer is 42"))
}
