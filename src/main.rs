mod opcode;
mod value;
mod vm;
mod assembler;

use assembler::Assembler;
use vm::VM;
use std::fs;

fn main() {
    println!("RustCode - A Custom Bytecode Language");
    println!("======================================\n");
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // Run a file
        let filename = &args[1];
        match run_file(filename) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Interactive mode
        println!("Running demo program...\n");
        run_demo();
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let mut assembler = Assembler::new();
    let bytecode = assembler.assemble(&source)?;
    
    let mut vm = VM::new(bytecode);
    vm.run()?;
    
    Ok(())
}

fn run_demo() {
    // Demo 1: Basic arithmetic
    println!("Demo 1: Basic Arithmetic (5 + 3 * 2)");
    let program1 = r#"
        PUSH 5
        PUSH 3
        PUSH 2
        MUL
        ADD
        PRINT
        HALT
    "#;
    
    execute_demo(program1);
    
    // Demo 2: Conditional jump
    println!("\nDemo 2: Conditional Logic");
    let program2 = r#"
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
    "#;
    
    execute_demo(program2);
    
    // Demo 3: Factorial calculation (simple)
    println!("\nDemo 3: Simple Calculation (10 - 3)");
    let program3 = r#"
        PUSH 10
        PUSH 3
        SUB
        PRINT
        HALT
    "#;
    
    execute_demo(program3);
    
    // Demo 4: String operations
    println!("\nDemo 4: String Operations");
    let program4 = r#"
        PUSH "Hello, "
        PUSH "RustCode!"
        ADD
        PRINT
        HALT
    "#;
    
    execute_demo(program4);
}

fn execute_demo(source: &str) {
    let mut assembler = Assembler::new();
    match assembler.assemble(source) {
        Ok(bytecode) => {
            let mut vm = VM::new(bytecode);
            match vm.run() {
                Ok(_) => {}
                Err(e) => eprintln!("Runtime error: {}", e),
            }
        }
        Err(e) => eprintln!("Assembly error: {}", e),
    }
}
