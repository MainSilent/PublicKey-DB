pub enum Operation {
    Add,
    Sort,
    Find
}

#[allow(dead_code)]
pub struct Request {
    pub op: Operation,
    pub value: String
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
                _ => {
                    return Err("Invalid operation");
                }
            },
            value: request[1].to_string()
        })
    }
}