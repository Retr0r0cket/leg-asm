# Operations organization: opset (each set of 0-7), purpose/subset, operation {code, # of args}

OPCODES_DICT = {
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
        "code": 0b000000   
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
        "code": 0b000001
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
        "memory": {
            "mem_load": {
                "code": 5,
                "args": 0,
            },
            "mem_save": {
                "code": 6,
                "args": 1,
                "disable_destination": False
            },
        },
    },
    "code": 0b000010
}
    
def validate_opcodes(opcodes_dict):
    if not isinstance(opcodes_dict, dict):
        raise TypeError("opcodes must be a dictionary")
    
    sets = list(opcodes_dict.keys())
    subsets = [list(opcodes_dict[set].keys()) for set in sets]
    # Eliminate codes for sets, meant as compiler variables
    for subset in subsets:
        for key in subset:
            if key == "code":
                subset.remove(key)
    
    operations = {}
    i = 0
    for set in opcodes_dict:
        for subset in subsets[i]:
            opcode_code_list = []
            for operation in opcodes_dict[set][subset]:
                if not isinstance(opcodes_dict[set][subset][operation], dict):
                    raise TypeError(f"opcode must be a dictionary, not {type(operation)}")
                
                opcode_code = opcodes_dict[set][subset][operation]["code"]
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
                if opcodes_dict[set][subset][operation]["args"] not in [1, 2]:
                    raise ValueError(f"Invalid number of arguments for {operation}")
                    
                opcode_code_list.append(opcode_code)
                operations[operation] = opcodes_dict[set][subset][operation]
        i += 1
        
    return operations
    
    # Make sure no opcode name duplicates
    if len(set(operations)) != len(operations):
        raise ValueError("Duplicate opcode names found")

if __name__ == "__main__":
    validate_opcodes(OPCODES_DICT)

REG_LIST = ['reg0', 'reg1', 'reg2', 'reg3', 'reg4', 'reg5']
DEST_LIST = REG_LIST + ['counter']

def correctNumOfArgs(args: list, operation: str) -> bool:
    drop_counter = 0
    for arg in args:
        if arg == '_':
            drop_counter += 1
    return 2 - drop_counter == OPCODES_DICT[operation]['args']