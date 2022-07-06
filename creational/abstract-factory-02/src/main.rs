
trait AbstractProductA {
    fn useful_function_a(&self) -> String;

}
trait AbstractProductB {
    fn useful_function_b(&self) -> String;
    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String;
}

trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn AbstractProductA>;
    fn create_product_b(&self) -> Box<dyn AbstractProductB>;
}

struct ConcreteProductA1;

impl AbstractProductA for ConcreteProductA1 {
    fn useful_function_a(&self) -> String {
        String::from("The result of the product A1.")
    }
}

struct ConcreteProductA2;

impl AbstractProductA for ConcreteProductA2 {
    fn useful_function_a(&self) -> String {
        String::from("The result of the product A2.")
    }
}

struct ConcreteProductB1;

impl AbstractProductB for ConcreteProductB1 {
    fn useful_function_b(&self) -> String {
        String::from("The result of the product B1.")
    }

    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String {
        let s = collaborator.useful_function_a();
        format!("The result of the B1 collaborating with the ({})", s)
    }
}

struct ConcreteProductB2;

impl AbstractProductB for ConcreteProductB2 {
    fn useful_function_b(&self) -> String {
        String::from("The result of the product B2.")
    }

    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String {
        let s = collaborator.useful_function_a();
        format!("The result of the B2 collaborating with the ({})", s)
    }
}



struct ConcreteFactory1;

impl AbstractFactory for ConcreteFactory1 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA1)
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB1)
    }
}

struct ConcreteFactory2;

impl AbstractFactory for ConcreteFactory2 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA2)
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB2)
    }
}

fn main() {
    println!("Client: Testing client code with the first factory type...");
    let factory1 = ConcreteFactory1;

    let product_a = factory1.create_product_a();
    let product_b = factory1.create_product_b();

    println!("{}", product_b.useful_function_b());
    println!("{}", product_b.another_useful_function_b(product_a));

    println!();

    println!("Client: Testing the same client code with the second factory type...");
    let factory2 = ConcreteFactory2;

    let product_a = factory2.create_product_a();
    let product_b = factory2.create_product_b();

    println!("{}", product_b.useful_function_b());
    println!("{}", product_b.another_useful_function_b(product_a));
}
