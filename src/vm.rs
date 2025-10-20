use crate::opcode::OpCode;
use crate::value::Value;

/// RustCode Virtual Machine
pub struct VM {
    /// Program bytecode
    code: Vec<u8>,
    /// Instruction pointer
    ip: usize,
    /// Evaluation stack
    stack: Vec<Value>,
    /// Memory storage
    memory: Vec<Value>,
    /// Call stack for function calls
    call_stack: Vec<usize>,
    /// Execution state
    halted: bool,
}

impl VM {
    /// Create a new VM with the given bytecode
    pub fn new(code: Vec<u8>) -> Self {
        VM {
            code,
            ip: 0,
            stack: Vec::new(),
            memory: vec![Value::Null; 256], // 256 memory slots
            call_stack: Vec::new(),
            halted: false,
        }
    }
    
    /// Run the VM until halt
    pub fn run(&mut self) -> Result<(), String> {
        while !self.halted && self.ip < self.code.len() {
            self.step()?;
        }
        Ok(())
    }
    
    /// Execute one instruction
    pub fn step(&mut self) -> Result<(), String> {
        if self.ip >= self.code.len() {
            return Err("Instruction pointer out of bounds".to_string());
        }
        
        let opcode_byte = self.code[self.ip];
        let opcode = OpCode::from_byte(opcode_byte)
            .ok_or_else(|| format!("Invalid opcode: 0x{:02X}", opcode_byte))?;
        
        self.ip += 1;
        
        match opcode {
            OpCode::Push => {
                let value = self.read_value()?;
                self.stack.push(value);
            }
            OpCode::Pop => {
                self.pop()?;
            }
            OpCode::Dup => {
                let value = self.peek()?.clone();
                self.stack.push(value);
            }
            OpCode::Swap => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(b);
                self.stack.push(a);
            }
            OpCode::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.add(&b)?;
                self.stack.push(result);
            }
            OpCode::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.sub(&b)?;
                self.stack.push(result);
            }
            OpCode::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.mul(&b)?;
                self.stack.push(result);
            }
            OpCode::Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.div(&b)?;
                self.stack.push(result);
            }
            OpCode::Mod => {
                let b = self.pop()?.as_integer()?;
                let a = self.pop()?.as_integer()?;
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                self.stack.push(Value::Integer(a % b));
            }
            OpCode::Neg => {
                let a = self.pop()?;
                let result = match a {
                    Value::Integer(i) => Value::Integer(-i),
                    Value::Float(f) => Value::Float(-f),
                    _ => return Err("Cannot negate non-numeric value".to_string()),
                };
                self.stack.push(result);
            }
            OpCode::Eq => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(Value::Boolean(a == b));
            }
            OpCode::Ne => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(Value::Boolean(a != b));
            }
            OpCode::Lt => {
                let b = self.pop()?.as_float()?;
                let a = self.pop()?.as_float()?;
                self.stack.push(Value::Boolean(a < b));
            }
            OpCode::Le => {
                let b = self.pop()?.as_float()?;
                let a = self.pop()?.as_float()?;
                self.stack.push(Value::Boolean(a <= b));
            }
            OpCode::Gt => {
                let b = self.pop()?.as_float()?;
                let a = self.pop()?.as_float()?;
                self.stack.push(Value::Boolean(a > b));
            }
            OpCode::Ge => {
                let b = self.pop()?.as_float()?;
                let a = self.pop()?.as_float()?;
                self.stack.push(Value::Boolean(a >= b));
            }
            OpCode::And => {
                let b = self.pop()?.as_bool();
                let a = self.pop()?.as_bool();
                self.stack.push(Value::Boolean(a && b));
            }
            OpCode::Or => {
                let b = self.pop()?.as_bool();
                let a = self.pop()?.as_bool();
                self.stack.push(Value::Boolean(a || b));
            }
            OpCode::Not => {
                let a = self.pop()?.as_bool();
                self.stack.push(Value::Boolean(!a));
            }
            OpCode::Jump => {
                let addr = self.read_u16()? as usize;
                self.ip = addr;
            }
            OpCode::JumpIf => {
                let addr = self.read_u16()? as usize;
                let cond = self.pop()?.as_bool();
                if cond {
                    self.ip = addr;
                }
            }
            OpCode::JumpIfNot => {
                let addr = self.read_u16()? as usize;
                let cond = self.pop()?.as_bool();
                if !cond {
                    self.ip = addr;
                }
            }
            OpCode::Call => {
                let addr = self.read_u16()? as usize;
                self.call_stack.push(self.ip);
                self.ip = addr;
            }
            OpCode::Return => {
                if let Some(return_addr) = self.call_stack.pop() {
                    self.ip = return_addr;
                } else {
                    self.halted = true;
                }
            }
            OpCode::Load => {
                let addr = self.read_u8()? as usize;
                if addr >= self.memory.len() {
                    return Err(format!("Memory address out of bounds: {}", addr));
                }
                self.stack.push(self.memory[addr].clone());
            }
            OpCode::Store => {
                let addr = self.read_u8()? as usize;
                if addr >= self.memory.len() {
                    return Err(format!("Memory address out of bounds: {}", addr));
                }
                let value = self.pop()?;
                self.memory[addr] = value;
            }
            OpCode::Print => {
                let value = self.pop()?;
                println!("{}", value);
            }
            OpCode::Input => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| format!("Input error: {}", e))?;
                self.stack.push(Value::String(input.trim().to_string()));
            }
            OpCode::Halt => {
                self.halted = true;
            }
        }
        
        Ok(())
    }
    
    /// Read a byte from the code
    fn read_u8(&mut self) -> Result<u8, String> {
        if self.ip >= self.code.len() {
            return Err("Unexpected end of code".to_string());
        }
        let byte = self.code[self.ip];
        self.ip += 1;
        Ok(byte)
    }
    
    /// Read a 16-bit value from the code (big-endian)
    fn read_u16(&mut self) -> Result<u16, String> {
        let high = self.read_u8()? as u16;
        let low = self.read_u8()? as u16;
        Ok((high << 8) | low)
    }
    
    /// Read a value from the code
    fn read_value(&mut self) -> Result<Value, String> {
        let value_type = self.read_u8()?;
        match value_type {
            0x00 => Ok(Value::Null),
            0x01 => {
                // Integer (i64)
                let mut bytes = [0u8; 8];
                for byte in &mut bytes {
                    *byte = self.read_u8()?;
                }
                Ok(Value::Integer(i64::from_be_bytes(bytes)))
            }
            0x02 => {
                // Float (f64)
                let mut bytes = [0u8; 8];
                for byte in &mut bytes {
                    *byte = self.read_u8()?;
                }
                Ok(Value::Float(f64::from_be_bytes(bytes)))
            }
            0x03 => {
                // Boolean
                let b = self.read_u8()?;
                Ok(Value::Boolean(b != 0))
            }
            0x04 => {
                // String
                let len = self.read_u16()? as usize;
                let mut bytes = Vec::with_capacity(len);
                for _ in 0..len {
                    bytes.push(self.read_u8()?);
                }
                let string = String::from_utf8(bytes)
                    .map_err(|e| format!("Invalid UTF-8 string: {}", e))?;
                Ok(Value::String(string))
            }
            _ => Err(format!("Unknown value type: 0x{:02X}", value_type)),
        }
    }
    
    /// Pop a value from the stack
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }
    
    /// Peek at the top of the stack
    fn peek(&self) -> Result<&Value, String> {
        self.stack.last().ok_or_else(|| "Stack is empty".to_string())
    }
    
    /// Get the current stack for debugging
    #[allow(dead_code)]
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    
    /// Check if VM has halted
    #[allow(dead_code)]
    pub fn is_halted(&self) -> bool {
        self.halted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push_and_pop() {
        // PUSH 42, HALT
        let code = vec![
            0x01, // PUSH
            0x01, // Type: Integer
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, // Value: 42
            0xFF, // HALT
        ];
        
        let mut vm = VM::new(code);
        vm.run().unwrap();
        
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::Integer(42));
    }
    
    #[test]
    fn test_arithmetic() {
        // PUSH 5, PUSH 3, ADD, HALT
        let code = vec![
            0x01, // PUSH
            0x01, // Type: Integer
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, // Value: 5
            0x01, // PUSH
            0x01, // Type: Integer
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // Value: 3
            0x10, // ADD
            0xFF, // HALT
        ];
        
        let mut vm = VM::new(code);
        vm.run().unwrap();
        
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::Integer(8));
    }
    
    #[test]
    fn test_comparison() {
        // PUSH 5, PUSH 3, GT, HALT
        let code = vec![
            0x01, // PUSH
            0x01, // Type: Integer
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, // Value: 5
            0x01, // PUSH
            0x01, // Type: Integer
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // Value: 3
            0x24, // GT
            0xFF, // HALT
        ];
        
        let mut vm = VM::new(code);
        vm.run().unwrap();
        
        assert_eq!(vm.stack().len(), 1);
        assert_eq!(vm.stack()[0], Value::Boolean(true));
    }
}
