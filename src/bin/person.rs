pub struct Person {
    name: String,
    age: u32,
    children: u32,
    favorite_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Person {
    pub fn new(name: &str, age: u32, children: u32, favorite_color:Color) -> Person {
        Person {
            name: name.to_string(),
            age,
            children,
            favorite_color,
        }
    }

    pub fn print(&self) -> String {
        format!(
            "name={}, age={}, has {} children and favorite color is {:?}",
            self.name, self.age, self.children, self.favorite_color
        )
    }
}

fn main() {
    let p = Person::new("matt", 35, 4, Color::Green);
    println!("{}", p.print());
    let c = Color::Red;
    match c {
        Color::Red => println!("It is red"),
        Color::Blue => println!("It is blue"),
        Color::Green => println!("It is green"),
    }
}
