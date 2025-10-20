# RustCode

A custom bytecode language and virtual machine implemented in Rust.

## Overview

RustCode is a stack-based bytecode language with its own virtual machine (VM) and assembler. It supports arithmetic operations, conditional logic, memory operations, and more.

## Features

- **Stack-based architecture**: Operations work with values on a stack
- **Rich instruction set**: 30+ opcodes for various operations
- **Type system**: Supports integers, floats, booleans, strings, and null values
- **Assembler**: Human-readable assembly syntax that compiles to bytecode
- **Memory operations**: 256 memory slots for storing values
- **Control flow**: Jumps, conditional jumps, and function calls
- **I/O operations**: Print and input capabilities

## Installation

Make sure you have Rust installed, then:

```bash
git clone <repository-url>
cd rustcode
cargo build --release
```

## Usage

### Running the demo program:

```bash
cargo run
```

### Running a RustCode file:

```bash
cargo run <filename.rcode>
# or
./target/release/rustcode <filename.rcode>
```

### Examples:

```bash
cargo run examples/hello.rcode
cargo run examples/arithmetic.rcode
cargo run examples/conditional.rcode
cargo run examples/memory.rcode
```

## RustCode Assembly Language

### Instruction Set

#### Stack Operations
- `PUSH <value>` - Push a value onto the stack
- `POP` - Pop the top value from the stack
- `DUP` - Duplicate the top stack value
- `SWAP` - Swap the top two stack values

#### Arithmetic Operations
- `ADD` - Add top two values
- `SUB` - Subtract top two values
- `MUL` - Multiply top two values
- `DIV` - Divide top two values
- `MOD` - Modulo operation
- `NEG` - Negate top value

#### Comparison Operations
- `EQ` - Equal comparison
- `NE` - Not equal comparison
- `LT` - Less than
- `LE` - Less than or equal
- `GT` - Greater than
- `GE` - Greater than or equal

#### Logical Operations
- `AND` - Logical AND
- `OR` - Logical OR
- `NOT` - Logical NOT

#### Control Flow
- `JUMP <label>` - Unconditional jump to label
- `JUMPIF <label>` - Jump if top of stack is true
- `JUMPIFNOT <label>` - Jump if top of stack is false
- `CALL <label>` - Call a function at label
- `RETURN` - Return from function

#### Memory Operations
- `LOAD <address>` - Load value from memory address (0-255)
- `STORE <address>` - Store top of stack to memory address

#### I/O Operations
- `PRINT` - Print the top of stack
- `INPUT` - Read input from stdin

#### Special
- `HALT` - Stop execution

### Value Types

- **Integer**: `42`, `-10`, `0`
- **Float**: `3.14`, `-0.5`, `2.0`
- **Boolean**: `true`, `false`
- **String**: `"Hello, World!"`
- **Null**: `null`

### Comments

Lines starting with `;` are comments:

```
; This is a comment
PUSH 42  ; This is also a comment
```

### Labels

Labels are defined by ending a line with `:`:

```
loop_start:
    PUSH 1
    JUMP loop_start
```

## Example Programs

### Hello World

```assembly
PUSH "Hello, World!"
PRINT
HALT
```

### Simple Arithmetic

```assembly
; Calculate (10 + 5) * 2
PUSH 10
PUSH 5
ADD
PUSH 2
MUL
PRINT
HALT
```

### Conditional Logic

```assembly
PUSH 10
PUSH 5
GT
JUMPIFNOT else_branch
PUSH "10 is greater than 5"
JUMP end
else_branch:
PUSH "10 is not greater than 5"
end:
PRINT
HALT
```

### Memory Operations

```assembly
PUSH 42
STORE 0        ; Store 42 at memory address 0

PUSH 100
STORE 1        ; Store 100 at memory address 1

LOAD 0         ; Load from address 0
LOAD 1         ; Load from address 1
ADD            ; Add them
PRINT          ; Should print 142
HALT
```

## Architecture

### Components

1. **OpCode** (`src/opcode.rs`): Defines all bytecode instructions
2. **Value** (`src/value.rs`): Runtime value types and operations
3. **VM** (`src/vm.rs`): Virtual machine that executes bytecode
4. **Assembler** (`src/assembler.rs`): Converts assembly text to bytecode

### Virtual Machine

The VM uses a stack-based architecture:
- **Stack**: For computation and temporary values
- **Memory**: 256 slots for persistent storage
- **Call Stack**: For function calls and returns
- **Instruction Pointer**: Tracks current execution position

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Development

### Building

```bash
cargo build
```

### Running in debug mode

```bash
cargo run
```

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.