/// Bytecode opcodes for RustCode VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Stack operations
    Push = 0x01,    // Push constant onto stack
    Pop = 0x02,     // Pop value from stack
    Dup = 0x03,     // Duplicate top of stack
    Swap = 0x04,    // Swap top two stack values
    
    // Arithmetic operations
    Add = 0x10,     // Add top two values
    Sub = 0x11,     // Subtract top two values
    Mul = 0x12,     // Multiply top two values
    Div = 0x13,     // Divide top two values
    Mod = 0x14,     // Modulo operation
    Neg = 0x15,     // Negate top value
    
    // Comparison operations
    Eq = 0x20,      // Equal comparison
    Ne = 0x21,      // Not equal comparison
    Lt = 0x22,      // Less than comparison
    Le = 0x23,      // Less than or equal
    Gt = 0x24,      // Greater than comparison
    Ge = 0x25,      // Greater than or equal
    
    // Logical operations
    And = 0x30,     // Logical AND
    Or = 0x31,      // Logical OR
    Not = 0x32,     // Logical NOT
    
    // Control flow
    Jump = 0x40,    // Unconditional jump
    JumpIf = 0x41,  // Jump if top of stack is true
    JumpIfNot = 0x42, // Jump if top of stack is false
    Call = 0x43,    // Call function
    Return = 0x44,  // Return from function
    
    // Memory operations
    Load = 0x50,    // Load from memory
    Store = 0x51,   // Store to memory
    
    // I/O operations
    Print = 0x60,   // Print top of stack
    Input = 0x61,   // Read input
    
    // Special
    Halt = 0xFF,    // Halt execution
}

impl OpCode {
    /// Convert byte to opcode
    pub fn from_byte(byte: u8) -> Option<OpCode> {
        match byte {
            0x01 => Some(OpCode::Push),
            0x02 => Some(OpCode::Pop),
            0x03 => Some(OpCode::Dup),
            0x04 => Some(OpCode::Swap),
            0x10 => Some(OpCode::Add),
            0x11 => Some(OpCode::Sub),
            0x12 => Some(OpCode::Mul),
            0x13 => Some(OpCode::Div),
            0x14 => Some(OpCode::Mod),
            0x15 => Some(OpCode::Neg),
            0x20 => Some(OpCode::Eq),
            0x21 => Some(OpCode::Ne),
            0x22 => Some(OpCode::Lt),
            0x23 => Some(OpCode::Le),
            0x24 => Some(OpCode::Gt),
            0x25 => Some(OpCode::Ge),
            0x30 => Some(OpCode::And),
            0x31 => Some(OpCode::Or),
            0x32 => Some(OpCode::Not),
            0x40 => Some(OpCode::Jump),
            0x41 => Some(OpCode::JumpIf),
            0x42 => Some(OpCode::JumpIfNot),
            0x43 => Some(OpCode::Call),
            0x44 => Some(OpCode::Return),
            0x50 => Some(OpCode::Load),
            0x51 => Some(OpCode::Store),
            0x60 => Some(OpCode::Print),
            0x61 => Some(OpCode::Input),
            0xFF => Some(OpCode::Halt),
            _ => None,
        }
    }
    
    /// Convert opcode to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }
    
    /// Check if opcode requires an operand
    #[allow(dead_code)]
    pub fn has_operand(self) -> bool {
        matches!(self, OpCode::Push | OpCode::Jump | OpCode::JumpIf | 
                      OpCode::JumpIfNot | OpCode::Call | OpCode::Load | OpCode::Store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_opcode_conversion() {
        assert_eq!(OpCode::from_byte(0x01), Some(OpCode::Push));
        assert_eq!(OpCode::from_byte(0x10), Some(OpCode::Add));
        assert_eq!(OpCode::from_byte(0xFF), Some(OpCode::Halt));
        assert_eq!(OpCode::from_byte(0x00), None);
    }
    
    #[test]
    fn test_opcode_to_byte() {
        assert_eq!(OpCode::Push.to_byte(), 0x01);
        assert_eq!(OpCode::Add.to_byte(), 0x10);
        assert_eq!(OpCode::Halt.to_byte(), 0xFF);
    }
    
    #[test]
    fn test_has_operand() {
        assert!(OpCode::Push.has_operand());
        assert!(OpCode::Jump.has_operand());
        assert!(!OpCode::Add.has_operand());
        assert!(!OpCode::Halt.has_operand());
    }
}
