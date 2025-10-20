/// Runtime value types for RustCode VM
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Null,
}

impl Value {
    /// Convert value to integer
    pub fn as_integer(&self) -> Result<i64, String> {
        match self {
            Value::Integer(i) => Ok(*i),
            Value::Float(f) => Ok(*f as i64),
            Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(format!("Cannot convert {:?} to integer", self)),
        }
    }
    
    /// Convert value to float
    pub fn as_float(&self) -> Result<f64, String> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(format!("Cannot convert {:?} to float", self)),
        }
    }
    
    /// Convert value to boolean
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
        }
    }
    
    /// Add two values
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(format!("Cannot add {:?} and {:?}", self, other)),
        }
    }
    
    /// Subtract two values
    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!("Cannot subtract {:?} and {:?}", self, other)),
        }
    }
    
    /// Multiply two values
    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(format!("Cannot multiply {:?} and {:?}", self, other)),
        }
    }
    
    /// Divide two values
    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Integer(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Integer(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            _ => Err(format!("Cannot divide {:?} and {:?}", self, other)),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_value_arithmetic() {
        let a = Value::Integer(5);
        let b = Value::Integer(3);
        
        assert_eq!(a.add(&b).unwrap(), Value::Integer(8));
        assert_eq!(a.sub(&b).unwrap(), Value::Integer(2));
        assert_eq!(a.mul(&b).unwrap(), Value::Integer(15));
        assert_eq!(a.div(&b).unwrap(), Value::Integer(1));
    }
    
    #[test]
    fn test_value_conversion() {
        let v = Value::Integer(42);
        assert_eq!(v.as_integer().unwrap(), 42);
        assert_eq!(v.as_float().unwrap(), 42.0);
        assert!(v.as_bool());
        
        let v = Value::Boolean(false);
        assert!(!v.as_bool());
    }
    
    #[test]
    fn test_string_concat() {
        let a = Value::String("Hello, ".to_string());
        let b = Value::String("World!".to_string());
        
        assert_eq!(a.add(&b).unwrap(), Value::String("Hello, World!".to_string()));
    }
}
