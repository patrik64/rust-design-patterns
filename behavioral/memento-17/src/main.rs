// Memento pattern

// Originator has its state.

// Caretaker memorize Originator's previous states.

// Every Originator's state has Memento trait.

trait Memento {
    fn get_value(&self) -> usize;
}

trait Originator {
    fn generate_memento(&self) -> Box<dyn Memento>;
    fn restore_from_memento(&mut self, _: &dyn Memento);
}

trait Caretaker {
    fn add_memento(&mut self, _: Box<dyn Memento>);
    fn get_memento(&mut self, _: usize) -> &dyn Memento;
}



#[derive(Debug)]
struct ConcreteOriginator(usize);

impl Originator for ConcreteOriginator {
    fn generate_memento(&self) -> Box<dyn Memento> {
        Box::new(ConcreteMemento(self.0))
    }

    fn restore_from_memento(&mut self, m: &dyn Memento) {
        self.0 = m.get_value()
    }
}

struct ConcreteMemento(usize);
impl Memento for ConcreteMemento {
    fn get_value(&self) -> usize {
        self.0
    }
}

struct ConcreteCaretaker {
    history: Vec<Box<dyn Memento>>,
}

impl ConcreteCaretaker {
    fn new() -> ConcreteCaretaker {
        ConcreteCaretaker {
            history: Vec::new(),
        }
    }
}

impl Caretaker for ConcreteCaretaker {
    fn add_memento(&mut self, m: Box<dyn Memento>) {
        self.history.push(m)
    }

    fn get_memento(&mut self, index: usize) -> &dyn Memento {
        &*self.history[index]
    }
}

fn main() {
    let mut caretaker = ConcreteCaretaker::new();
    let mut originator = ConcreteOriginator(10);

    caretaker.add_memento(originator.generate_memento());
    println!("{:?}", originator);
    originator.0 = 99;
    println!("{:?}", originator);
    originator.restore_from_memento(caretaker.get_memento(0));
    println!("{:?}", originator);
}