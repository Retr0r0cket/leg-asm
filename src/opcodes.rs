use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Opcode {
    bit_code: u8,
    num_of_args: u8,
}

struct OpcodeSet {
    opcode_subset: Vec<Vec<Opcodes>>,
    bit_code: u8
}

impl iterator for OpcodeSet {
    type Item = Vec<Vec<Opcodes>>;

    fn next(&mut self) -> Option<Vec<Vec<Opcodes>>> {
        let mut next: Option<Vec<Vec<Opcodes>>> = None;
        if !self.opcodes.is_empty() {
            next = self.opcode_set.pop();
        }
        next
    }
}

struct OpcodeMasterList {
    opcode_master_list: Vec<OpcodeSet>,
    bit_code: u8
}

impl iterator for OpcodeMasterList {
    type Item = Vec<OpcodeSet>;

    fn next(&mut self) -> Option<Vec<OpcodeSet>> {
        let mut next: Option<Vec<OpcodeSet>> = None;
        if !self.opcodes.is_empty() {
            next = self.opcode_master_list.pop();
        }
        next
    }
}