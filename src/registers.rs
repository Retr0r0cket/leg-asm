use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Registers {
    data_registers: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Iterator for Registers {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut next = None;
        if !self.data_registers.is_empty() {
            next = self.data_registers.pop();
        } else if !self.inputs.is_empty() {
            next = self.inputs.pop();
        } else if !self.outputs.is_empty() {
            next = self.outputs.pop();
        }
        next
    }
}

impl Registers {
    pub fn check_for_duplicates (&mut self) -> bool {
        let mut duplicates = false;
        let mut seen: Vec<&String> = Vec::new();
        for register in self.data_registers.iter().chain(self.inputs.iter()).chain(self.outputs.iter()) {
            if seen.contains(&register) {
                duplicates = true;
                break;
            } else {
                seen.push(register);
            }
        }
        duplicates
    }

    pub fn check_for_uppercase (&mut self) -> bool {
        let mut case = false;
        for register in self.data_registers.iter().chain(self.inputs.iter()).chain(self.outputs.iter()) {
            if register.chars().any(char::is_uppercase) {
                case = true;
                break;
            }
        }
        case
    }
}
