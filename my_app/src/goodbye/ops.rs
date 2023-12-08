pub struct Ops {
  name: String,
  operations: Vec<String>,
}

impl Ops {
  pub fn new(name: String, operations: Vec<String>) -> Ops {
    Ops {
      name,
      operations,
    }
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_operations(&self) -> &Vec<String> {
    &self.operations
  }
}