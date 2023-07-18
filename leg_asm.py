from sys import argv
from re import match

import lasm_lib

# Editable variables
# Just don't change INSTRUCTIONS_SIZE or INSTRUCTION_NUMBER_START bc those are linked to the architecture itself

LEG_ASM_EXTENSION = '.lasm'
LEG_MACHINE_CODE_EXTENSION = '.lmc'

LEG_ASM_COMMENT = '#'
LEG_ASM_JUMP_KEYWORD = 'label'
LEG_ASM_CONST_KEYWORD = 'const'

INSTRUCTIONS_SIZE = 4
INSTRUCTION_NUMBER_START = 1

# First one is opcode, second is both arguments, final is destination
OUTPUT_NUMER_FORMATS = ('bin', 'dec', 'dec')

# Better to catch sooner rather than later
operations = lasm_lib.validate_opcodes(lasm_lib.OPCODES_DICT)

asm_file = argv[1]
if not argv[1].endswith(LEG_ASM_EXTENSION):
    print(f"Warning: incorrect file extension used. Expected: {LEG_ASM_EXTENSION}")

with open(asm_file, 'r') as f:
    lines = f.readlines().rstrip()

jump_labels = {}
const_labels = {}
# Check syntax for jump labels
i = 0
for line in lines:
    # Jump labels
    if line.startswith(LEG_ASM_JUMP_KEYWORD):
        line_text = line.split()
        if len(line_text) != 2:
            raise SyntaxError(f"Invalid jump syntax length on line {i}/in-game line {i*4}: {line}")
        if line_text[0] != 'label':
            raise SyntaxError(f"Invalid jump syntax on line {i}/in-game line {i*4}: {line}")
        if not match(r"^[\w\d_]*", line_text[1]):
            raise ValueError(f"Invalid label name: {line_text[1]}. All characters must be alphanumeric or underscore")
        if line_text[1] in jump_labels:
            raise KeyError(f"Duplicate jump label found: key {line_text[1]} found at both line {jump_labels[line_text[1]]}/in-game line{jump_labels[line_text[2]]*4} and line {i}/in-game line{i*4}")
        jump_labels[line_text[1]] = i
        
    # Constants
    if line.startswith(LEG_ASM_CONST_KEYWORD):
        line_text = line.split()
        if len(line_text) != 3:
            raise SyntaxError(f"Invalid jump syntax length on line {i}/in-game line {i*4}: {line}")
        if line_text[1] != 'const':
            raise SyntaxError(f"Invalid jump syntax on line {i}/in-game line {i*4}: {line}")
        if not match(r"^[\w\d_]*", line_text[1]):
            raise ValueError(f"Invalid const name: {line_text[1]}. All characters must be alphanumeric or underscore")
        try:
            int(line_text[2])
        except ValueError:
            raise ValueError(f"Invalid const value on line {i}: {line_text[2]}. Must be an integer")
        if line_text[1] in const_labels:
            raise KeyError(f"Duplicate jump label found: key {line_text[1]} found at both line {const_labels[line_text[1]]}/in-game line{const_labels[line_text[1]]*4} and line {i}/in-game line{i*4}")
        const_labels[line_text[1]] = i
    i += 1

# Now that we have a list of all consts and jumps, time to go for instructions

im1_flags = []
im2_flags = []
i = 0
for line in lines:
    # opcode
    line_text = line.split()
    if len(line_text) != 4:
        raise SyntaxError(f"Invalid syntax length on line {i}/in-game line {i*4}: {line}")
    if line_text[0] not in operations:
        raise ValueError(f"Invalid operation on line {i}/in-game line {i*4}: {line_text[0]}")
    
    # Arg verification
    if not lasm_lib.correctNumOfArgs(line_text[1:2], line_text[0]):
        raise ValueError(f"Invalid number of arguments on line {i}/in-game line {i*4}: {line}")
    
    n = 0
    for arg in line_text[1:2]:
        try:
            int(arg)
            if n == 0:
                im1_flags.append(i)
            elif n == 1:
                im2_flags.append(i)
            else:
                raise IndexError(f"Invalid index when iterating over args: {n}")
        except ValueError:
            if str(arg) not in lasm_lib.REG_LIST:
                raise ValueError(f"Invalid arg value on line {i}/in-game line {i*4}: {arg}. Must be an integer or register")
        n += 1
        
    # Destination verification
    while True:
        if "disable_destination" in operations[arg[0]]:
            if operations[arg[0]]["disable_destination"] == True:
                if arg[2] != '_':
                    break
                else:
                    raise ValueError(f"Invalid destination value on line {i}/in-game line {i*4}: {arg[2]}. No destination for {arg[0]}")
        else:
            if arg[2] not in lasm_lib.DEST_LIST:
                raise ValueError(f"Invalid destination value on line {i}/in-game line {i*4}: {arg[2]}. Must be a register or counter")
            break
    i += 1

# Now for the assembely part

''' args to add: 
stdout vs to file
output file name (defaults to {asm_file}.lmc)
validate opcodes
autoscan for files
configuration file
'''

''' features to add:
if only one arg and arg 2 is immediate, handle like it should
'''