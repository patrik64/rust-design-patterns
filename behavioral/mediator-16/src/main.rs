use std::rc::Rc;
use std::cell::RefCell;

struct Component1 {
    mediator: Option<Rc<dyn Mediator>>,
}

impl Component1 {

    fn set_mediator(&mut self, m: Rc<dyn Mediator>) {
        self.mediator = Some(m);
    }

    fn do_a(&self) {
        println!("Component 1 does A.");
        self.mediator.as_ref().unwrap().notify(String::from("A"));
    }

    fn do_b(&self) {
        println!("Component 1 does B.");
        self.mediator.as_ref().unwrap().notify(String::from("B"));
    }
}

struct Component2 {
    mediator: Option<Rc<dyn Mediator>>,
}

impl Component2 {

    fn set_mediator(&mut self, m: Rc<dyn Mediator>) {
        self.mediator = Some(m);
    }

    fn do_c(&self) {
        println!("Component 2 does C.");
        self.mediator.as_ref().unwrap().notify(String::from("C"));
    }

    fn do_d(&self) {
        println!("Component 2 does D.");
        self.mediator.as_ref().unwrap().notify(String::from("D"));
    }
}

trait Mediator {
    fn notify(&self, event: String);
}

struct ConcreteMediator {
    component1: Rc<RefCell<Component1>>,
    component2: Rc<RefCell<Component2>>,
}

impl Mediator for ConcreteMediator {

    fn notify(&self, event: String) {
        if event == *"A" {
            println!("Mediator reacts on A and triggers following operations:");
            self.component2.borrow().do_c();
        }
        
        if event == *"D" {
            println!("Mediator reacts on D and triggers following operations:");
            self.component1.borrow().do_b();
            self.component2.borrow().do_c();
        }
    }
}

impl ConcreteMediator {
    fn new(c1: Rc<RefCell<Component1>>, c2: Rc<RefCell<Component2>>) -> ConcreteMediator {
        ConcreteMediator { component1: c1, component2: c2 }
    }
}

fn main() {
    let c1 = Rc::new(RefCell::new(Component1 { mediator: None }));
    let c2 = Rc::new(RefCell::new(Component2 { mediator: None }));

    let cc1 = Rc::clone(&c1);
    let cc2 = Rc::clone(&c2);

    let mediator = Rc::new(ConcreteMediator::new(cc1, cc2));
    let rcmed = Rc::clone(&mediator);
    

    c1.borrow_mut().set_mediator(mediator);
    c2.borrow_mut().set_mediator(rcmed);
    
    println!("Client triggers operation A.");
    c1.borrow().do_a();

    println!();
    println!("Client triggers operation D.");
    c2.borrow().do_d();
    
}