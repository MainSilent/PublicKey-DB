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

        Ok(Self {
            op: match request[0] {
                "add" => Operation::Add,
                "sort" => Operation::Sort,
                "find" => Operation::Find,
                _ => {
                    return Err("Invalid operation");
                }
            },
            value: match request.get(1) {
                Some(value) => value.to_string(),
                None => "".to_string()
            }
        })
    }
}