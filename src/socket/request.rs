pub enum Operation {
    Add,
    Find,
    Remove
}

pub struct Request {
    op: Operation,
    value: String
}

impl Request {
    pub fn new(raw_request: &String) -> Result<Self, &str> {
        let request: Vec<&str> = raw_request.split_ascii_whitespace().collect();
        if request.len() != 2 {
            return Err("Invalid number of parameters");
        }

        Ok(Self {
            op: match request[0] {
                "add" => Operation::Add,
                "find" => Operation::Find,
                "remove" => Operation::Remove,
                _ => {
                    return Err("Invalid operation");
                }
            },
            value: request[1].to_string()
        })
    }
}