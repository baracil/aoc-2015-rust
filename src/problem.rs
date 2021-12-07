use std::{io, result};
use std::fs::read_to_string;

pub enum Part {
    Part1,
    Part2
}

pub struct Problem {
    input_filename:String
}

pub type AOCResult<T> = result::Result<T, String>;


impl Problem {

    pub fn factory(for_test:bool) -> impl FnOnce(u32) -> Problem {
        if for_test {
            Problem::test_of_day
        } else {
            Problem::of_day
        }
    }

    pub fn of_day(day:u32) -> Self {
        let input_filename = form_filename(day);
        Self{input_filename}
    }

    #[allow(dead_code)]
    pub fn test_of_day(day:u32) -> Self {
        let input_filename = form_test_filename(day);
        Self{input_filename}
    }

    #[allow(dead_code)]
    pub fn read_input(&self) -> AOCResult<String> {
        let content : io::Result<String> = read_to_string(self.input_filename.to_string());
        match content {
            Ok(content) => AOCResult::Ok(content),
            Err(err) => AOCResult::Err(err.to_string())
        }
    }

    #[allow(dead_code)]
    pub fn read_input_as_mapped_lines<U, F: FnMut(&str) -> U>(&self, op:F) -> AOCResult<Vec<U>> {
        Ok(self.read_input()?
            .split("\n")
            .map(op)
            .collect())
    }

    #[allow(dead_code)]
    pub fn read_input_as_vec_of_lines(&self) -> AOCResult<Vec<String>> {
        self.read_input().map(to_vec_of_line)
    }

    #[allow(dead_code)]
    pub fn read_input_as_vec_of_u32(&self) -> AOCResult<Vec<u32>> {
        self.read_input().map(to_vec_of_u32)
    }
}



pub fn to_vec_of_line(content: String) -> Vec<String> {
    content.split("\n")
        .map(|s| s.to_string())
        .collect()
}

pub fn to_vec_of_u32(content: String) -> Vec<u32> {
    content.split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn form_filename(day: u32) -> String {
    return format!("./input/problem/day_{number:0width$}.txt", number = day, width = 2);
}

fn form_test_filename(day: u32) -> String {
    return format!("./input/test/day_{number:0width$}.txt", number = day, width = 2);
}