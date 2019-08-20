pub struct Hello;

impl Hello {
    pub fn hi() -> String {
        format!("Hi")
    }

    pub fn hey(&self) -> String {
        format!("Hey")
    }

    pub fn hello(&self) -> String {
        format!("Hello")
    }
}

struct World;

impl World {
    pub fn world() -> String {
        format!("World")
    }

    pub fn wooorld(&self) -> String {
        format!("Woooooorld")
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

    pub fn greet(&self, hi: String) -> String {
        let world = World::world();                     // static call
        format!("{} {}.", hi, world)
    }

    pub fn greet_excited(&self, hi: String) -> String{
        let w = World;
        let world = w.wooorld();                        // method call
        let formatted = self.format();                  // private inherent method call
        format!("{} {}{}", hi, world, formatted)
    }

    fn format(&self) -> String {
        format!("{}!", self.0)
    }
}
