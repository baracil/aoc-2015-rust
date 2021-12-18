pub enum Previous {
    None,
    Some(char, u32),
}

pub struct Reader {
    previous: Previous,
    result: String,
}

impl Default for Reader {
    fn default() -> Self {
        Reader {
            previous: Previous::None,
            result: "".to_string(),
        }
    }
}

impl Reader {
    pub fn push_char(&mut self, c: char) {
        use Previous::*;
        match &self.previous {
            None => self.previous = Some(c, 1),

            Some(previous, count) => {
                if *previous == c {
                    self.previous = Some(c, count + 1)
                } else {
                    self.result
                        .push_str(format!("{}{}", count, previous).as_str());
                    self.previous = Some(c, 1);
                }
            }
        }
    }

    pub fn build(&mut self) -> &str {
        if let Previous::Some(c, count) = self.previous {
            self.result.push_str(format!("{}{}", count, c).as_str());
        }
        &self.result
    }
}

pub fn read(line: &str) -> String {
    let mut reader = line.chars().fold(Reader::default(), |mut acc, c| {
        acc.push_char(c);
        acc
    });
    reader.build().to_string()
}
