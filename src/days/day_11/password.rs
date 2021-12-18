pub struct Password {
    password: Vec<u8>,
}

const NB_LETTERS: u8 = 26;
const A_AS_U8: u8 = b'a';

const I_DIGIT: u8 = b'i' - A_AS_U8;
const O_DIGIT: u8 = b'o' - A_AS_U8;
const L_DIGIT: u8 = b'l' - A_AS_U8;

impl Password {
    pub fn new(line: &str) -> Self {
        let password = line.chars().map(|c| c as u8 - A_AS_U8).collect();
        Password { password }
    }

    pub fn generate_next_valid(&mut self) -> String {
        loop {
            self.generate_next();
            if self.is_requirement2_fulfil() && self.are_requirement1_and_3_fulfil() {
                let result = self
                    .password
                    .iter()
                    .map(|u| (u + A_AS_U8) as char)
                    .collect::<String>();
                return result;
            }
        }
    }

    fn generate_next(&mut self) {
        let mut idx = self.password.len() - 1;

        loop {
            let digit = self.password[idx] + 1;
            if digit >= NB_LETTERS {
                self.password[idx] = 0;
            } else {
                self.password[idx] = digit;
                break;
            }
            idx -= 1;
        }
    }
    //ghjaabcc
    pub fn are_requirement1_and_3_fulfil(&self) -> bool {
        let mut req1_ok = false;
        let mut previous_pair_index = 0;
        let mut nb_pairs = 0;
        let mut match_in_a_row = 0;

        for (idx, pair) in self.password.windows(2).enumerate() {
            let prev = pair[0];
            let this = pair[1];

            if prev == this && previous_pair_index + 1 != idx {
                nb_pairs += 1;
                previous_pair_index = idx;
            }

            if prev + 1 == this {
                match_in_a_row += 1;
                if match_in_a_row == 2 {
                    req1_ok = true;
                }
            } else {
                match_in_a_row = 0;
            }
        }

        req1_ok && nb_pairs >= 2
    }

    pub fn is_requirement2_fulfil(&self) -> bool {
        !self
            .password
            .iter()
            .any(|c| *c == I_DIGIT || *c == O_DIGIT || *c == L_DIGIT)
    }
}
