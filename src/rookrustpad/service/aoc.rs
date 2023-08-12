use std::collections::HashMap;
use std::error::Error;
use std::fs::read_dir;

use lazy_regex::regex;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod y2022;
#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AocAnswer {
    year: u32,
    day: u32,
    part: u8,
    result: String,
    log: String,
    execution_time: u32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AocProblem {
    year: u32,
    day: u32,
    part: u8,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AocInput {
    year: u32,
    day: u32,
    part: u8,
    input: String,
}

type AocFunctionResult = Result<String, Box<dyn Error>>;
type AocResult = Result<AocAnswer, Box<dyn Error>>;

pub type AocFunction = fn(input_path: String, log: String) -> AocFunctionResult;

#[derive(Debug)]
pub struct AocService {
    input_directory: String,
    problem_answers: HashMap<AocProblem, AocFunction>,
}

impl AocService {
    pub fn new(input_directory: String) -> AocService {
        AocService {
            input_directory: input_directory,
            problem_answers: HashMap::new(),
        }
    }

    pub fn create_default(input_directory: String) -> AocService {
        let mut service = AocService::new(input_directory);
        y2022::configure_service(&mut service);
        service
    }

    pub fn register_answer(&mut self, year: u32, day: u32, part: u8, answer: AocFunction) {
        self.problem_answers.insert(
            AocProblem {
                year: year,
                day: day,
                part: part,
            },
            answer,
        );
    }

    pub fn list_problems(&self) -> Vec<AocProblem> {
        self.problem_answers.keys().cloned().collect()
    }

    pub fn list_inputs(&self) -> Result<Vec<AocInput>, Box<dyn Error>> {
        let year_re = regex!(r"^\d{4}$");
        let f_name_re = regex!(r"^day_(\d+)_(\d+)(?:_(.+))?.txt$");

        let mut inputs: Vec<AocInput> = vec![];

        let dir_h = read_dir(&self.input_directory)?;
        for dir_ent in dir_h {
            let dir_ent = dir_ent?;
            let ft = dir_ent.file_type()?;
            let file_name = dir_ent.file_name();
            let file_name = file_name.to_str().ok_or("Invalid file name")?;

            if ft.is_dir() && year_re.is_match(file_name) {
                let year = file_name.parse::<u32>()?;
                let sub_dir_h = read_dir(dir_ent.path())?;

                println!("Found year: {}", year);

                for sub_dir_ent in sub_dir_h {
                    let sub_dir_ent = sub_dir_ent?;
                    let sub_file_name = sub_dir_ent.file_name();
                    let sub_file_name = sub_file_name.to_str().ok_or("Invalid file name")?;

                    println!("looking at: {}", sub_file_name);

                    if let Some(caps) = f_name_re.captures(sub_file_name) {
                        let day = caps
                            .get(1)
                            .ok_or("Invalid capture")?
                            .as_str()
                            .parse::<u32>()?;
                        let part = caps
                            .get(2)
                            .ok_or("Invalid capture")?
                            .as_str()
                            .parse::<u8>()?;

                        inputs.push(AocInput {
                            year: year,
                            day: day,
                            part: part,
                            input: sub_dir_ent
                                .path()
                                .to_str()
                                .ok_or("Invalid path")?
                                .to_string(),
                        });
                    }
                }
            }
        }

        Ok(inputs)
    }
}
