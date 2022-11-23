use std::io::stdin;


//#![warn(clippy::all, clippy::pediantic)]
fn main() {
    what_is_your_name();
    let_visitorlist(&String::from("bert"));
    let_visitorlist(&String::from("sample"));
}

struct Visitor {
    name: String, 
    greeting: String,
}

struct Greetings {
    Vec<String> names;
}

impl Greetings {
    pub fn sayGreetings(&mut self, name:String) {
            self.names.push(name.clone());
    }
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    println!("what is your name?");
    let mut your_name = String::new(); 
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line"); 
    println!("Hello {}", your_name);
    your_name
        .trim()
        .to_lowercase()
}

fn let_visitorlist(item: &String) {
    // let visitor_list = ["bert", "steve"]; 
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge"),
        Visitor::new("fred", "Wow who invited Fred!"),
    ];

    // for visitor in &visitor_list {
    //     if visitor == item {
    //         println!("item is here");
    //     } else {
    //         println!("no item is here");
    //     }
    // } 
}


