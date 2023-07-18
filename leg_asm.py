from sys import argv
from re import match

# Editable variables
# Just don't change INSTRUCTIONS_SIZE or INSTRUCTION_NUMBER_START bc those are linked to the architecture itself

LEG_ASM_EXTENSION = '.lasm'
LEG_MACHINE_CODE_EXTENSION = '.lmc'

LEG_ASM_COMMENT = '#'
LEG_ASM_JUMP_SECTION_KEYWORD = 'label'

INSTRUCTIONS_SIZE = 4
INSTRUCTION_NUMBER_START = 1

# First one is opcode, second is both arguments, final is destination
OUTPUT_NUMER_FORMATS = ('bin', 'dec', 'dec')

asm_file = argv[1]
if not argv[1].endswith(LEG_ASM_EXTENSION):
    print(f"Warning: incorrect file extension used. Expected: {LEG_ASM_EXTENSION}")

with open(asm_file, 'r') as f:
    lines = f.readlines().rstrip()

# Check syntax for jump labels
i = 0
for line in lines:
    if line.startswith(LEG_ASM_JUMP_SECTION_KEYWORD):
        line_text = line.split()
        if line_text[1] != 'label' or len(line_text) != 2:
            raise SyntaxError(f"Invalid jump syntax on line {i}/in-game line {i*4}: {line}")
        if not match(r"^[\w\d_]*", line_text[2]):
            raise ValueError(f"Invalid label name: {line_text[2]}. All characters must be alphanumeric or underscore")
    i += 1

''' args to add: 
stdout vs to file
output file name (defaults to {asm_file}.lmc)
validate opcodes
autoscan for files
configuration file
'''

''' features to add:
if only one arg and arg 2 is immediate, handle like it should
enforce dropping if only one arg
const handling
'''