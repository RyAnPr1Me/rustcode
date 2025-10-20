use crate::opcode::OpCode;

/// RustCode Assembler - converts assembly text to bytecode
pub struct Assembler {
    labels: std::collections::HashMap<String, usize>,
    unresolved_jumps: Vec<(usize, String)>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: std::collections::HashMap::new(),
            unresolved_jumps: Vec::new(),
        }
    }
    
    /// Assemble source code into bytecode
    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();
        
        // First pass: collect labels
        for (_line_num, line) in source.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            
            if line.ends_with(':') {
                let label = line.trim_end_matches(':').to_string();
                self.labels.insert(label, bytecode.len());
            } else {
                // Estimate bytecode length for this instruction
                let (instruction, args) = self.parse_instruction_line(line)?;
                if !instruction.is_empty() {
                    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                    let estimated_size = self.estimate_instruction_size(&instruction, &arg_refs)?;
                    bytecode.resize(bytecode.len() + estimated_size, 0);
                }
            }
        }
        
        // Second pass: generate bytecode
        bytecode.clear();
        for (line_num, line) in source.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.ends_with(':') {
                continue;
            }
            
            self.assemble_instruction(line, &mut bytecode, line_num + 1)?;
        }
        
        // Resolve jump labels
        for (offset, label) in &self.unresolved_jumps {
            let addr = self.labels.get(label)
                .ok_or_else(|| format!("Undefined label: {}", label))?;
            
            // Write address as big-endian u16
            bytecode[*offset] = (*addr >> 8) as u8;
            bytecode[*offset + 1] = (*addr & 0xFF) as u8;
        }
        
        Ok(bytecode)
    }
    
    fn estimate_instruction_size(&self, instruction: &str, args: &[&str]) -> Result<usize, String> {
        let opcode = self.parse_opcode(instruction)?;
        
        match opcode {
            OpCode::Push => {
                // 1 byte opcode + value encoding
                Ok(1 + self.estimate_value_size(args.get(0).ok_or("PUSH requires an argument")?))
            }
            OpCode::Jump | OpCode::JumpIf | OpCode::JumpIfNot | OpCode::Call => {
                Ok(3) // 1 byte opcode + 2 bytes address
            }
            OpCode::Load | OpCode::Store => {
                Ok(2) // 1 byte opcode + 1 byte address
            }
            _ => Ok(1), // Just the opcode
        }
    }
    
    fn estimate_value_size(&self, value_str: &str) -> usize {
        if value_str.starts_with('"') {
            // String: 1 type + 2 length + string bytes
            let s = value_str.trim_matches('"');
            1 + 2 + s.len()
        } else if value_str.contains('.') {
            // Float: 1 type + 8 bytes
            9
        } else if value_str == "true" || value_str == "false" {
            // Boolean: 1 type + 1 byte
            2
        } else {
            // Integer: 1 type + 8 bytes
            9
        }
    }
    
    fn assemble_instruction(&mut self, line: &str, bytecode: &mut Vec<u8>, line_num: usize) -> Result<(), String> {
        let (instruction, args) = self.parse_instruction_line(line)?;
        if instruction.is_empty() {
            return Ok(());
        }
        
        let opcode = self.parse_opcode(&instruction)?;
        bytecode.push(opcode.to_byte());
        
        match opcode {
            OpCode::Push => {
                if args.is_empty() {
                    return Err(format!("Line {}: PUSH requires an argument", line_num));
                }
                self.encode_value(&args[0], bytecode)?;
            }
            OpCode::Jump | OpCode::JumpIf | OpCode::JumpIfNot | OpCode::Call => {
                if args.is_empty() {
                    return Err(format!("Line {}: {} requires a label", line_num, instruction));
                }
                
                let label = args[0].clone();
                if let Some(&addr) = self.labels.get(&label) {
                    bytecode.push((addr >> 8) as u8);
                    bytecode.push((addr & 0xFF) as u8);
                } else {
                    // Save for later resolution
                    self.unresolved_jumps.push((bytecode.len(), label));
                    bytecode.push(0);
                    bytecode.push(0);
                }
            }
            OpCode::Load | OpCode::Store => {
                if args.is_empty() {
                    return Err(format!("Line {}: {} requires an address", line_num, instruction));
                }
                let addr = args[0].parse::<u8>()
                    .map_err(|e| format!("Line {}: Invalid address: {}", line_num, e))?;
                bytecode.push(addr);
            }
            _ => {
                // No arguments needed
            }
        }
        
        Ok(())
    }
    
    fn parse_instruction_line(&self, line: &str) -> Result<(String, Vec<String>), String> {
        let line = line.trim();
        
        // Find first space or tab
        let split_pos = line.find(|c: char| c.is_whitespace());
        
        if let Some(pos) = split_pos {
            let instruction = line[..pos].to_string();
            let rest = line[pos..].trim();
            
            // Check if argument is a quoted string
            if rest.starts_with('"') {
                // Find the closing quote
                if let Some(end_quote) = rest[1..].find('"') {
                    let arg = rest[..=end_quote + 1].to_string();
                    Ok((instruction, vec![arg]))
                } else {
                    Err("Unterminated string literal".to_string())
                }
            } else {
                // Split by whitespace for non-string arguments
                let args: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                Ok((instruction, args))
            }
        } else {
            // No arguments
            Ok((line.to_string(), vec![]))
        }
    }
    
    fn parse_opcode(&self, instruction: &str) -> Result<OpCode, String> {
        let upper = instruction.to_uppercase();
        match upper.as_str() {
            "PUSH" => Ok(OpCode::Push),
            "POP" => Ok(OpCode::Pop),
            "DUP" => Ok(OpCode::Dup),
            "SWAP" => Ok(OpCode::Swap),
            "ADD" => Ok(OpCode::Add),
            "SUB" => Ok(OpCode::Sub),
            "MUL" => Ok(OpCode::Mul),
            "DIV" => Ok(OpCode::Div),
            "MOD" => Ok(OpCode::Mod),
            "NEG" => Ok(OpCode::Neg),
            "EQ" => Ok(OpCode::Eq),
            "NE" => Ok(OpCode::Ne),
            "LT" => Ok(OpCode::Lt),
            "LE" => Ok(OpCode::Le),
            "GT" => Ok(OpCode::Gt),
            "GE" => Ok(OpCode::Ge),
            "AND" => Ok(OpCode::And),
            "OR" => Ok(OpCode::Or),
            "NOT" => Ok(OpCode::Not),
            "JUMP" => Ok(OpCode::Jump),
            "JUMPIF" => Ok(OpCode::JumpIf),
            "JUMPIFNOT" => Ok(OpCode::JumpIfNot),
            "CALL" => Ok(OpCode::Call),
            "RETURN" => Ok(OpCode::Return),
            "LOAD" => Ok(OpCode::Load),
            "STORE" => Ok(OpCode::Store),
            "PRINT" => Ok(OpCode::Print),
            "INPUT" => Ok(OpCode::Input),
            "HALT" => Ok(OpCode::Halt),
            _ => Err(format!("Unknown instruction: {}", instruction)),
        }
    }
    
    fn encode_value(&self, value_str: &str, bytecode: &mut Vec<u8>) -> Result<(), String> {
        if value_str.starts_with('"') && value_str.ends_with('"') {
            // String
            bytecode.push(0x04); // String type
            let s = value_str.trim_matches('"');
            let bytes = s.as_bytes();
            bytecode.push((bytes.len() >> 8) as u8);
            bytecode.push((bytes.len() & 0xFF) as u8);
            bytecode.extend_from_slice(bytes);
        } else if value_str == "true" {
            bytecode.push(0x03); // Boolean type
            bytecode.push(1);
        } else if value_str == "false" {
            bytecode.push(0x03); // Boolean type
            bytecode.push(0);
        } else if value_str == "null" {
            bytecode.push(0x00); // Null type
        } else if value_str.contains('.') {
            // Float
            let f = value_str.parse::<f64>()
                .map_err(|e| format!("Invalid float: {}", e))?;
            bytecode.push(0x02); // Float type
            bytecode.extend_from_slice(&f.to_be_bytes());
        } else {
            // Integer
            let i = value_str.parse::<i64>()
                .map_err(|e| format!("Invalid integer: {}", e))?;
            bytecode.push(0x01); // Integer type
            bytecode.extend_from_slice(&i.to_be_bytes());
        }
        
        Ok(())
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;
    use crate::value::Value;
    
    #[test]
    fn test_simple_program() {
        let source = r#"
            PUSH 5
            PUSH 3
            ADD
            HALT
        "#;
        
        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(source).unwrap();
        
        let mut vm = VM::new(bytecode);
        vm.run().unwrap();
        
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::Integer(8));
    }
    
    #[test]
    fn test_labels_and_jumps() {
        let source = r#"
            PUSH 1
            JUMPIF skip
            PUSH 999
        skip:
            PUSH 42
            HALT
        "#;
        
        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(source).unwrap();
        
        let mut vm = VM::new(bytecode);
        vm.run().unwrap();
        
        // Should skip PUSH 999
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::Integer(42));
    }
    
    #[test]
    fn test_string_value() {
        let source = r#"
            PUSH "Hello, World!"
            HALT
        "#;
        
        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(source).unwrap();
        
        let mut vm = VM::new(bytecode);
        vm.run().unwrap();
        
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::String("Hello, World!".to_string()));
    }
}
