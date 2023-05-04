use std::{cell::RefCell, rc::Rc};

use chrono::Local;

trait Colleague {
    fn update(&self);
    fn send_message(&self, msg: &str);
    fn name(&self) -> String;
}
struct ConcreteColleagueA {
    mediator: Rc<RefCell<dyn Mediator>>,
    name: String,
}
impl ConcreteColleagueA {
    fn new(mediator: Rc<RefCell<dyn Mediator>>, name: String) -> Self {
        ConcreteColleagueA { mediator, name }
    }
}
impl Colleague for ConcreteColleagueA {
    fn update(&self) {
        println!("update by ConcreteColleagueA");
    }
    fn send_message(&self, msg: &str) {
        self.mediator.borrow().show_message(msg, self);
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

trait Mediator {
    fn add_colleague(&mut self, colleague: Rc<dyn Colleague>);
    fn show_message(&self, msg: &str, colleague: &dyn Colleague);
}

struct ChatRoom {
    colleagues: Vec<Rc<dyn Colleague>>,
}

impl ChatRoom {
    fn new() -> Self {
        ChatRoom {
            colleagues: Vec::new(),
        }
    }
}

impl Mediator for ChatRoom {
    fn add_colleague(&mut self, colleague: Rc<dyn Colleague>) {
        self.colleagues.push(colleague);
    }
    fn show_message(&self, msg: &str, colleague: &dyn Colleague) {
        println!(
            "{} [{}]:{}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            colleague.name(),
            msg
        );
    }
}

fn main() {
    let mediator = Rc::new(RefCell::new(ChatRoom::new()));

    let colleague_a = Rc::new(ConcreteColleagueA::new(
        mediator.clone(),
        "white donkey".to_string(),
    ));
    let colleague_b = Rc::new(ConcreteColleagueA::new(
        mediator.clone(),
        "black donkey".to_string(),
    ));
    mediator.borrow_mut().add_colleague(colleague_a.clone());
    mediator.borrow_mut().add_colleague(colleague_b.clone());
    colleague_a.send_message("Hi all");
    colleague_b.send_message("Hi");
}
