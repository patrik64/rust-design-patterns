use std::rc::Rc;

trait Visitor {
    fn visit_concrete_component_a(&self, element: &ConcreteComponentA);
    fn visit_concrete_component_b(&self, element: &ConcreteComponentB);
}

struct ConcreteVisitor1;

impl Visitor for ConcreteVisitor1 {
    fn visit_concrete_component_a(&self, element: &ConcreteComponentA) {
        println!("{} + ConcreteVisitor1", element.exclusive_method_of_concrete_component_a());
    }

    fn visit_concrete_component_b(&self, element: &ConcreteComponentB) {
        println!("{} + ConcreteVisitor1", element.special_method_of_concrete_component_b());
    }
}

struct ConcreteVisitor2;

impl Visitor for ConcreteVisitor2 {

    fn visit_concrete_component_a(&self, element: &ConcreteComponentA) {
        println!("{} + ConcreteVisitor2", element.exclusive_method_of_concrete_component_a());
    }

    fn visit_concrete_component_b(&self, element: &ConcreteComponentB) {
        println!("{} + ConcreteVisitor2", element.special_method_of_concrete_component_b());
    }
}

trait Component {
    fn accept(&self, visitor: Rc<dyn Visitor>);
}

struct ConcreteComponentA;

impl ConcreteComponentA {
    fn exclusive_method_of_concrete_component_a(&self) -> String {
        String::from("A")
    }
}

impl Component for ConcreteComponentA {
    fn accept(&self, visitor: Rc<dyn Visitor>) {
        visitor.visit_concrete_component_a(&*self);
    }
}

struct ConcreteComponentB;

impl ConcreteComponentB {
    fn special_method_of_concrete_component_b(&self) -> String {
        String::from("B")
    }
}

impl Component for ConcreteComponentB {
    fn accept(&self, visitor: Rc<dyn Visitor>) {
        visitor.visit_concrete_component_b(&*self);
    }
}




fn main() {

    let c1 = ConcreteComponentA;
    let c2 = ConcreteComponentB;

    println!("The client code works with all visitors via the base Visitor interface:");
    let v1 = Rc::new(ConcreteVisitor1);
    let v1rc = Rc::clone(&v1);
    
    c1.accept(v1);
    c2.accept(v1rc);
    
    println!();

    println!("It allows the same client code to work with different types of visitors:");
    let v2 = Rc::new(ConcreteVisitor2);
    let v2rc = Rc::clone(&v2);
    
    c1.accept(v2);
    c2.accept(v2rc);
    
}
