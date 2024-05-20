pub type Value = f64;

pub fn print_value(value: Value) {
    print!("{:e}", value);
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
