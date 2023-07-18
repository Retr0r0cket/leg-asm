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

if len(argv) == 3:
    raise ValueError("Too many/few arguments. Please specify output file or remove -o flag")

if len(argv) == 1:
    raise ValueError("Too few arguments. Please specify source file")

if len(argv) == 2:
    output_file = argv[1].replace(LEG_ASM_EXTENSION, LEG_MACHINE_CODE_EXTENSION)
    
if len(argv) == 4:
    output_file = argv[3]

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
line_number = 0
for line in lines:
    # Jump labels
    if line.startswith(LEG_ASM_JUMP_KEYWORD):
        line_text = line.split()
        if len(line_text) != 2:
            raise SyntaxError(f"Invalid jump syntax length on line {line_number}/in-game line {line_number*4}: {line}")
        if line_text[0] != 'label':
            raise SyntaxError(f"Invalid jump syntax on line {line_number}/in-game line {line_number*4}: {line}")
        if not match(r"^[\w\d_]*", line_text[1]):
            raise ValueError(f"Invalid label name: {line_text[1]}. All characters must be alphanumeric or underscore")
        if line_text[1] in jump_labels:
            raise KeyError(f"Duplicate jump label found: key {line_text[1]} found at both line {jump_labels[line_text[1]]}/in-game line{jump_labels[line_text[2]]*4} and line {line_number}/in-game line{line_number*4}")
        jump_labels[line_text[1]] = line_number
        
    # Constants
    if line.startswith(LEG_ASM_CONST_KEYWORD):
        line_text = line.split()
        if len(line_text) != 3:
            raise SyntaxError(f"Invalid jump syntax length on line {line_number}/in-game line {line_number*4}: {line}")
        if line_text[1] != 'const':
            raise SyntaxError(f"Invalid jump syntax on line {line_number}/in-game line {line_number*4}: {line}")
        if not match(r"^[\w\d_]*", line_text[1]):
            raise ValueError(f"Invalid const name: {line_text[1]}. All characters must be alphanumeric or underscore")
        try:
            int(line_text[2])
        except ValueError:
            raise ValueError(f"Invalid const value on line {line_number}: {line_text[2]}. Must be an integer")
        if line_text[1] in const_labels:
            raise KeyError(f"Duplicate jump label found: key {line_text[1]} found at both line {const_labels[line_text[1]]}/in-game line{const_labels[line_text[1]]*4} and line {line_number}/in-game line{line_number*4}")
        const_labels[line_text[1]] = line_number
    line_number += 1

# Now that we have a list of all consts and jumps, time to go for instructions

im1_flags = []
im2_flags = []
line_number = 0
for line in lines:
    # opcode
    line_text = line.split()
    if len(line_text) != 4:
        raise SyntaxError(f"Invalid syntax length on line {line_number}/in-game line {line_number*4}: {line}")
    if line_text[0] not in operations:
        raise ValueError(f"Invalid operation on line {line_number}/in-game line {line_number*4}: {line_text[0]}")
    
    # Arg verification
    if not lasm_lib.correctNumOfArgs(line_text[1:2], line_text[0]):
        raise ValueError(f"Invalid number of arguments on line {line_number}/in-game line {line_number*4}: {line}")
    
    arg_number = 0
    for arg in line_text[1:2]:
        try:
            int(arg)
            if arg_number == 0:
                im1_flags.append(line_number)
            elif arg_number == 1:
                im2_flags.append(line_number)
            else:
                raise IndexError(f"Invalid index when iterating over args: {arg_number}")
        except ValueError:
            if str(arg) not in lasm_lib.INPUT_LIST:
                raise ValueError(f"Invalid arg value on line {line_number}/in-game line {line_number*4}: {arg}. Must be an integer, register, or sysinput")
        arg_number += 1
        
    # Destination verification
    while True:
        if "disable_destination" in operations[arg[0]]:
            if operations[arg[0]]["disable_destination"] == True:
                if arg[3] != '_':
                    break
                else:
                    raise ValueError(f"Invalid destination value on line {line_number}/in-game line {line_number*4}: {arg[2]}. No destination for {arg[0]}")
        elif operations[arg[0]] in lasm_lib.OPCODES_DICT["set1"]["cond"]:
            if arg[3] not in jump_labels:
                try:
                    int(arg)
                except ValueError:
                    raise ValueError(f"Invalid destination value on line {line_number}/in-game line {line_number*4}: {arg[2]}. Must be integer for condition")
        else:
            if arg[3] not in lasm_lib.DEST_LIST:
                raise ValueError(f"Invalid destination value on line {line_number}/in-game line {line_number*4}: {arg[2]}. Must be a register, counter, or sysout")
            break
    line_number += 1

# Now for the assembely part
with open(output_file, 'w') as f:
    line_number = 0
    for line in lines:
        if line.startswith(LEG_ASM_JUMP_KEYWORD) or line.startswith(LEG_ASM_CONST_KEYWORD) or line.startswith(LEG_ASM_COMMENT) or line == '':
            f.write(f"{line}\n")
        line_text = line.split()
        opcode = lasm_lib.OPCODES_DICT[operations[line_text[0]]]["code"] + operations[line_text[0]]["code"]
        if line_number in im1_flags:
            opcode += 128
        if line_number in im2_flags:
            opcode += 64
        
        arg1 = lasm_lib.argument_binary_value(line_text[1])
        arg2 = lasm_lib.argument_binary_value(line_text[2])
        
        if operations[line_text[0]] in lasm_lib.OPCODES_DICT["set1"]["cond"]:
            dest = line_text[3]
        else:
            dest = lasm_lib.DEST_LIST.index(line_text[3])
        
        f.write(f"{hex(opcode)} {hex(arg1)} {hex(arg2)} {hex(dest)}\n")
        line_number += 1