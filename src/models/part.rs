pub struct Part {
    name: String,
}

impl Part {
    pub fn new(name: String) -> Part {
        Part { name }
    }

    pub fn get_symbol(&self) -> String {
        String::from(self.name.chars().next().unwrap())
    }
}
