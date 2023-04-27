// 一个接收者？

use std::{cell::RefCell, rc::Rc};

/// 命令接口
trait Command {
    fn execute(&self);
    fn undo(&self);
}

#[derive(Debug)]
enum Status {
    On,
    Off,
}

/// 命令接收者
struct LightReceiver {
    status: Status,
}

impl LightReceiver {
    fn new() -> Self {
        LightReceiver {
            status: Status::Off,
        }
    }
    fn on(&mut self) {
        self.status = Status::On;
        println!("电灯打开了！");
    }

    fn off(&mut self) {
        self.status = Status::Off;
        println!("电灯关闭了！");
    }
}

struct TvReceiver {
    status: Status,
}

impl TvReceiver {
    fn new() -> Self {
        TvReceiver {
            status: Status::Off,
        }
    }
    fn on(&mut self) {
        self.status = Status::On;
        println!("电视机打开了！");
    }

    fn off(&mut self) {
        self.status = Status::Off;
        println!("电视机关闭了！");
    }
}

/// 命令实现
struct LightOnCommand {
    receiver: Rc<RefCell<LightReceiver>>,
}

impl LightOnCommand {
    fn new(receiver: Rc<RefCell<LightReceiver>>) -> Self {
        LightOnCommand { receiver }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.receiver.borrow_mut().on();
    }

    fn undo(&self) {
        self.receiver.borrow_mut().off();
    }
}

struct LightOffCommand {
    receiver: Rc<RefCell<LightReceiver>>,
}

impl LightOffCommand {
    fn new(receiver: Rc<RefCell<LightReceiver>>) -> Self {
        LightOffCommand { receiver }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.receiver.borrow_mut().off();
    }

    fn undo(&self) {
        self.receiver.borrow_mut().on();
    }
}

struct TvOnCommand {
    receiver: Rc<RefCell<TvReceiver>>,
}

impl TvOnCommand {
    fn new(receiver: Rc<RefCell<TvReceiver>>) -> Self {
        TvOnCommand { receiver }
    }
}

impl Command for TvOnCommand {
    fn execute(&self) {
        self.receiver.borrow_mut().on();
    }

    fn undo(&self) {
        self.receiver.borrow_mut().off();
    }
}

struct TvOffCommand {
    receiver: Rc<RefCell<TvReceiver>>,
}

impl TvOffCommand {
    fn new(receiver: Rc<RefCell<TvReceiver>>) -> Self {
        TvOffCommand { receiver }
    }
}

impl Command for TvOffCommand {
    fn execute(&self) {
        self.receiver.borrow_mut().off();
    }

    fn undo(&self) {
        self.receiver.borrow_mut().on();
    }
}

struct NoCommand;

impl Command for NoCommand {
    fn execute(&self) {}

    fn undo(&self) {}
}

/// 命令调用者
struct RemoteControl {
    on_commands: Vec<Rc<Box<dyn Command>>>,
    off_commands: Vec<Rc<Box<dyn Command>>>,
    undo_command: Option<Rc<Box<dyn Command>>>,
}

impl RemoteControl {
    fn new() -> Self {
        RemoteControl {
            on_commands: Vec::new(),
            off_commands: Vec::new(),
            undo_command: None,
        }
    }
    fn set_command(&mut self, on_command: Rc<Box<dyn Command>>, off_command: Rc<Box<dyn Command>>) {
        self.on_commands.push(on_command);
        self.off_commands.push(off_command);
    }

    fn on_button_was_pushed(&mut self, index: usize) {
        match self.on_commands.get(index) {
            Some(x) => {
                x.execute();
                self.undo_command = Some(x.clone());
            }
            _ => {
                println!("fail");
            }
        }
    }
    fn off_button_was_pushed(&mut self, index: usize) {
        match self.off_commands.get(index) {
            Some(x) => {
                x.execute();
                self.undo_command = Some(x.clone());
            }
            _ => {
                println!("fail");
            }
        }
    }

    fn undo_button_was_pushed(&mut self) {
        match &self.undo_command {
            Some(x) => {
                x.undo();
                self.undo_command = None;
            }
            _ => {
                println!("---")
            }
        }
    }
}

fn main() {
    let light_receiver = Rc::new(RefCell::new(LightReceiver::new()));
    let light_on_command: Rc<Box<dyn Command>> =
        Rc::new(Box::new(LightOnCommand::new(light_receiver.clone())));
    let light_off_command: Rc<Box<dyn Command>> =
        Rc::new(Box::new(LightOffCommand::new(light_receiver)));

    let tv_receiver = Rc::new(RefCell::new(TvReceiver::new()));
    let tv_on_command: Rc<Box<dyn Command>> =
        Rc::new(Box::new(TvOnCommand::new(tv_receiver.clone())));
    let tv_off_command: Rc<Box<dyn Command>> = Rc::new(Box::new(TvOffCommand::new(tv_receiver)));

    let mut remote_control = RemoteControl::new();
    remote_control.set_command(light_on_command, light_off_command);
    remote_control.set_command(tv_on_command, tv_off_command);

    remote_control.on_button_was_pushed(0);
    remote_control.off_button_was_pushed(0);
    remote_control.undo_button_was_pushed();
    remote_control.on_button_was_pushed(1);
    remote_control.off_button_was_pushed(1);
    remote_control.undo_button_was_pushed();
}
