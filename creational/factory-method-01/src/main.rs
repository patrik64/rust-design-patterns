trait Product {
    fn operation(&self) -> String;
}

struct ConcreteProduct1;

impl Product for ConcreteProduct1 {

    fn operation(&self) -> String {
        "{Result of ConcreteProduct1}".to_string()
    }
}

impl ConcreteProduct1 {
    fn new() -> ConcreteProduct1 {
        ConcreteProduct1 {}
    }
}

struct ConcreteProduct2;

impl Product for ConcreteProduct2 {
    fn operation(&self) -> String {
        "{Result of ConcreteProduct2}".to_string()
    }
}

impl ConcreteProduct2 {
    fn new() -> ConcreteProduct2 {
        ConcreteProduct2 {}
    }
}

trait Creator {
    fn factory_method(&self) -> Box<dyn Product>;

    fn some_operation(&self) -> String {
        let product = self.factory_method();
        format!("Creator: The same creator's code has just worked with {}", product.operation())
    }
}
struct ConcreteCreator1;

impl Creator for ConcreteCreator1 {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProduct1::new())
    }
}

struct ConcreteCreator2;

impl Creator for ConcreteCreator2 {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProduct2::new())
    }
}


fn main() {

    println!("App: Launched with the ConcreteCreator1.");
    let creator1 = ConcreteCreator1;

    println!("Client: I'm not aware of the creator's class, but it still works.");
    println!("{}", creator1.some_operation());

    println!();

    println!("App: Launched with the ConcreteCreator2.");
    let creator2 = ConcreteCreator2;

    println!("Client: I'm not aware of the creator's class, but it still works.");
    println!("{}", creator2.some_operation());
}
