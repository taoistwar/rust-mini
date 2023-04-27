struct Request {
    content: String,
    level: i32,
}

trait Handler {
    fn handle(&self, req: &Request) -> Option<String>;
}

struct ConcreteHandler {
    level: i32,
    next: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandler {
    fn handle(&self, request: &Request) -> Option<String> {
        if request.level <= self.level {
            Some(format!(
                "Handled by handler of level {}, {}\n",
                self.level, request.content
            ))
        } else {
            self.next
                .as_ref()
                .and_then(|handler| handler.handle(request))
        }
    }
}

fn main() {
    let handler1 = ConcreteHandler {
        level: 3,
        next: None,
    };
    let handler2 = ConcreteHandler {
        level: 2,
        next: Some(Box::new(handler1)),
    };
    let handler3 = ConcreteHandler {
        level: 1,
        next: Some(Box::new(handler2)),
    };
    let request1 = Request {
        content: "Request of level 2".to_string(),
        level: 2,
    };
    let request2 = Request {
        content: "Request of level 4".to_string(),
        level: 4,
    };
    let result1 = handler3.handle(&request1);
    println!("{:?}", result1);
    println!("---");
    let result2 = handler3.handle(&request2);
    println!("{:?}", result2);
}
