enum Operation {
    Add,
    Find,
    Remove
}

pub struct Request {
    op: Operation,
    value: String
}

impl Request {
    pub fn new(req: &String) -> Self {
        Self {
            op: Operation::Add,
            value: "sdf".to_string()
        }
    }
}