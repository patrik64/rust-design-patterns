pub trait FlyBehavior {
    fn fly(&self);
}

pub struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying!");
    }
}

pub struct FlyNoWay;
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly");
    }
}

pub struct FlyRocketPowered;
impl FlyBehavior for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket");
    }
}

pub trait QuackBehavior {
    fn quack(&self);
}

pub struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

pub struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}

pub struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Squeak");
    }
}


pub struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}

impl Duck {
    pub fn new() -> Self {
        Duck {
            fly_behavior: Box::new(FlyNoWay),
            quack_behavior: Box::new(MuteQuack),
        }
    }

    pub fn set_fly_behavior(&mut self, fly_behavior: Box<dyn FlyBehavior>) {
        self.fly_behavior = fly_behavior;
    }

    pub fn set_quack_behavior(&mut self, quack_behavior: Box<dyn QuackBehavior>) {
        self.quack_behavior = quack_behavior;
    }

    pub fn perform_fly(&self) {
        self.fly_behavior.fly();
    }

    pub fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
}

fn main() {
    let mut duck = Duck::new();
    duck.perform_fly();
    duck.perform_quack();
    println!("----------");
    duck.set_fly_behavior(Box::new(FlyWithWings));
    duck.perform_fly();
    println!("----------");
    duck.set_fly_behavior(Box::new(FlyWithWings));
    duck.set_quack_behavior(Box::new(Squeak));
    duck.perform_quack();
}
