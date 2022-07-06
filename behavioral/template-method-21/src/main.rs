pub trait CaffeineBeverage {
    fn prepare_recipe(&self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        self.add_condiment();
    }
    fn brew(&self);
    fn add_condiment(&self);
    fn boil_water(&self) {
        println!("Boiling water");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup");
    }
}

pub struct Tea;

impl CaffeineBeverage for Tea {
    fn brew(&self) {
        println!("Steeping the tea");
    }

    fn add_condiment(&self) {
        println!("Adding Lemon");
    }
}

pub struct Coffee;

impl CaffeineBeverage for Coffee {
    fn brew(&self) {
        println!("Dripping Coffee through filter");
    }

    fn add_condiment(&self) {
       println!("Adding Sugar and Milk");
    }
}

fn main() {
    let tea = Tea;
    let coffee = Coffee;

    println!("\nMaking tea...");
    tea.prepare_recipe();

    println!("\nMaking coffee...");
    coffee.prepare_recipe();
}