pub struct Hello;

impl Hello {
    pub fn hi() -> String {
        format!("Hi")
    }

    pub fn hello(&self) -> String {
        format!("Hello")
    }
}

pub struct HelloWorld(String);

impl HelloWorld {
    pub fn new(s: String) -> Self {
        HelloWorld(s)
    }

    pub fn update(&mut self, s: String) {
        self.0 = s;
    }

    pub fn greet(&self, hi: &String) -> String {
        // Instance method call: std::string::ToString::to_string
        format!("{} {} {}", hi, "world".to_string(), self.0)
    }

    pub fn greet_excited(&mut self, hi: &String) -> String {
        // Inherent private method call: structs::lib::helloworld::HelloWorld::format
        self.0 = self.format();

        // Inherent public method call: structs::lib::helloworld::HelloWorld::greet
        self.greet(hi)
    }

    fn format(&self) -> String {
        format!("{}!", self.0)
    }
}
