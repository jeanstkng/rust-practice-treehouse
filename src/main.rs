use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut visitor_name = String::new();

    stdin()
        .read_line(&mut visitor_name)
        .expect("Failed to read line");
    visitor_name.trim().to_lowercase()
}

fn how_old_question() -> i8 {
    let mut visitor_age = String::new();

    stdin()
        .read_line(&mut visitor_age)
        .expect("Failed to read line");
    match visitor_age.parse::<i8>() {
        Ok(int) => return int,
        Err(_err) => return 0,
    }
}

fn main() {
    let mut visitors = vec![
        Visitor::new("jean", VisitorAction::Accept, 20),
        Visitor::new("chris", VisitorAction::Refuse, 29),
        Visitor::new(
            "cristo",
            VisitorAction::AcceptWithNote {
                note: String::from("Viva cristo rey omg"),
            },
            15,
        ),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitors.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("...how old are you? (Leave empty and press ENTER to quit)");
                    let age = how_old_question();
                    println!("{} is not on the visitor list.", name);
                    visitors.push(Visitor::new(&name, VisitorAction::Probation, age))
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitors);
}
