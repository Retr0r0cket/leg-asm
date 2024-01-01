# leg-asm

## About
Leg-asm is an assembler for [my version of the leg architecture](www.github.com/Retr0r0cket/leg) from the game Turing Complete written in Rust. I started work on this because I couldn't be bothered to add 3 characters of text to indicate if I was using an immediate value in an operation, so I decided to have another program do it for me along with do basic syntax checks.

## Installation:
- Binary
    - Download the binary from releases.
    - Add the binary to your path
- Source
    - Identical except you download and compile the source code
    - The funtions being used are dependent on the toolchain you want to use. I wanted to use the split_whitespaces() function, but that is currently unstable. As an exercise in macro use, you can compile it with either nightly or stable toolchains, but they won't use the exact same functions. Otherwise, compiles as normal.

## Usage
- To assemble a program, just run '''lasm {assembly file}'''
    - This will result in a machine code file ending in a .lmc extension with the same name as your assembly program in the same directory.
- To assemble a program and dictate the file extension and directory of the machine code, use the -o flag
    - '''lasm {assembly file} -o {machine code file}'''

## Configuration
All variables are listed below:

### Machine Code Options
#### Output Mode
- The in-game assembly editor (which you cannot avoid using, so everything can be run in it) can operate at all times using hexidecimal. It can also work with plaintext **but only if the in game assembly definitions are up to date** (which is usually the case). 
- Use hexidecimal for reliability, plaintext for readability
#### Remove_empty_lines
- Removes all empty lines in asssembly. Recommended to keep this on to make it easier to determine where functions start and stop in machine code, **ESPECIALLY WHEN PLAINTEXT IS ENABLED**.
#### Absolute Jump Paths
- If enabled, if any jumps to labels are found, it will remove the labels from the machine code and substitude in the line number of the jump label in the machine code.
- Recommended to disable with plaintext because the entire point is reliability, and enforced for use with hexidecimal (or else what is the point of using hex if you're just going to get plaintext).
    - If you for some reason really want to use this (which normally is only activated by mistake), you can pass '''-ignore-formatting''' as a command line argument last.

### JSON Options
Note: I would not change these options unless you modify the arcitecture or I somehow forget to update the assembler with new opcodes and registers
#### Use Embedded Json
- Utilizes the JSON data embedded into the binary at compile time.
- Shouldn't be a problem, but just in case you need to edit a file, I'd recommend keeping it off. Shouldn't be an issue if enabled though.
#### Opcode JSON Location
- Is the filepath to the opcode JSON containing a list of every opcode that LEG can executes
- Works as a relative or absolute path
- Is ignored if embedded JSON is on
#### Registers JSON Location
- Operates the same as the opcode JSON, but for the JSON containing all possible registers one can write to