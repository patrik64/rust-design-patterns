pub trait ComponentUnit {
    fn get_strength(&self) -> i32;
    fn add_unit(&mut self, unit: Box<dyn ComponentUnit>);
    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>>;
    fn get_unit(&self, index: usize) -> &dyn ComponentUnit;
    fn remove(&mut self, index: usize);
}

pub struct Archer;
pub struct Infantryman;
pub struct Horseman;

impl ComponentUnit for Archer {
    fn get_strength(&self) -> i32 {
        1
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}

impl ComponentUnit for Infantryman {
    fn get_strength(&self) -> i32 {
        2
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}

impl ComponentUnit for Horseman {
    fn get_strength(&self) -> i32 {
        3
    }

    fn add_unit(&mut self, _: Box<dyn ComponentUnit>) {
        unimplemented!()
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        unimplemented!()
    }

    fn get_unit(&self, _: usize) -> &dyn ComponentUnit {
        unimplemented!()
    }

    fn remove(&mut self, _: usize) {
        unimplemented!()
    }
}

pub struct CompositeUnit {
    units: Vec<Box<dyn ComponentUnit>>,
}

impl CompositeUnit {
    pub fn new() -> CompositeUnit {
        CompositeUnit { units: Vec::new() }
    }
}

impl Default for CompositeUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentUnit for CompositeUnit {
    fn get_strength(&self) -> i32 {
        let mut res = 0;

        for unit in &self.units {
            res += unit.get_strength();
        }

        res
    }

    fn add_unit(&mut self, unit: Box<dyn ComponentUnit>) {
        self.units.push(unit);
    }

    fn get_units(&self) -> &Vec<Box<dyn ComponentUnit>> {
        &self.units
    }

    fn get_unit(&self, index: usize) -> &dyn ComponentUnit {
        &*self.units[index]
    }

    fn remove(&mut self, index: usize) {
        self.units.remove(index);
    }
}

fn create_legion() -> CompositeUnit {
    let mut composite_legion = CompositeUnit::new();

    for _ in 0..3000 {
        composite_legion.add_unit(Box::new(Infantryman));
    }

    for _ in 0..1200 {
        composite_legion.add_unit(Box::new(Archer));
    }

    for _ in 0..300 {
        composite_legion.add_unit(Box::new(Horseman));
    }

    composite_legion
}
fn main() {
    let mut army = CompositeUnit::new();

    for _ in 0..4 {
        let legion = create_legion();
        army.add_unit(Box::new(legion));
    }

    {
        let legions = army.get_units();
        println!("Roman army damaging strength is {}", army.get_strength());
        println!("Roman army has {} legions\n", legions.len());
    }

    {
        let legion = army.get_unit(0);
        println!(
            "Roman legion damaging strength is {}",
            legion.get_strength()
        );
        println!("Roman legion has {} units\n", legion.get_units().len());
    }

    army.remove(0);

    let legions = army.get_units();
    println!("Roman army damaging strength is {}", army.get_strength());
    println!("Roman army has {} legions", legions.len());
}