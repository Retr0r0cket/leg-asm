# Operations organization: opset (each set of 0-7), purpose/subset, operation {code, # of args}

VALID_OPERATIONS = {
    "set0": {
        "math": {
            "add": {
                "code": 0,
                "args": 2,
            },
            "sub": {
                "code": 1,
                "args": 2,
            },
            "mult_low": {
                "code": 6,
                "args": 2,
            },
            "mult_up": {
                "code": 7,
                "args": 2,
            }
        },
        "logic": {
            "and": {
                "code": 2,
                "args": 2,
            },
            "or": {
                "code": 3,
                "args": 2,
            },
            "not": {
                "code": 4,
                "args": 1,
            },
            "xor": {
                "code": 5,
                "args": 2,
            },
        },
        "code": 0b00   
    },
    
    "set1": {
        "cond": {
            "equal": {
                "code": 0,
                "args": 2,
            },
            "not_equal": {
                "code": 1,
                "args": 2,
            },
            "less": {
                "code": 2,
                "args": 2,
            },
            "less_or_equal": {
                "code": 3,
                "args": 2,
            },
            "greater": {
                "code": 4,
                "args": 2,
            },
            "greater_or_equal": {
                "code": 5,
                "args": 2,
            },
        },
        "div": {
            "div_product": {
                "code": 6,
                "args": 2,
            },
            "div_mod_2": {
                "code": 7,
                "args": 2,
            },
        },
        "code": 0b01
    },
    
    "set2": {
        "shift": {
            "a_shift_r": {
                "code": 0,
                "args": 2,
            },
            "roll_r": {
                "code": 1,
                "args": 2,
            },
            "roll_l": {
                "code": 2,
                "args": 2,
            },
            "shift_r": {
                "code": 3,
                "args": 2,
            },
            "shift ": {
                "code": 4,
                "args": 2,
            },
        },
        "heap": {
            "heap_load": {
                "code": 5,
                "args": 1,
            },
            "heap_save": {
                "code": 6,
                "args": 1,
            },
        },
    },
    "code": 0b10
}
    
def validate_opcodes(opcodes_list):
    if not isinstance(opcodes_list, dict):
        raise TypeError("opcodes must be a dictionary")
    
    sets = list(opcodes_list.keys())
    subsets = [list(opcodes_list[set].keys()) for set in sets]
    # Eliminate codes for sets, meant as compiler variables
    for subset in subsets:
        for key in subset:
            if key == "code":
                subset.remove(key)
    
    operations = []
    i = 0
    for set in opcodes_list:
        for subset in subsets[i]:
            opcode_code_list = []
            for operation in opcodes_list[set][subset]:
                if not isinstance(opcodes_list[set][subset][operation], dict):
                    raise TypeError(f"opcode must be a dictionary, not {type(operation)}")
                
                opcode_code = opcodes_list[set][subset][operation]["code"]
                # code handling
                if not isinstance(opcode_code, int):
                    raise TypeError(f"opcode code must be an integer, not {type(opcode_code)}")
                if not 0 <= opcode_code <= 7:
                    raise ValueError(f"opcode code must be between 0 and 7 inclusive, not {opcode_code}")
                # Make sure no opcode code duplicates or else compiler won't work
                if opcode_code in opcode_code_list:
                    raise ValueError(f"Duplicate opcode code found in {subset}")
                from re import match
                if not match(r"^[\w\d_]*", operation):
                    raise ValueError(f"Invalid opcode name: {operation}. All characters must be alphanumeric or underscore")
                    
                opcode_code_list.append(opcode_code)
                operations.append(operation)
        i += 1
    
    # Make sure no opcode name duplicates
    if len(set(operations)) != len(operations):
        raise ValueError("Duplicate opcode names found")

if __name__ == "__main__":
    validate_opcodes(VALID_OPERATIONS)