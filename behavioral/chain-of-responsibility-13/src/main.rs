pub trait Policeman {
    fn set_next(&mut self, next: Box<dyn Policeman>);
    fn investigate(&self, crime: u8);
}

pub struct Detective {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
    name: String,
}

impl Detective {
    pub fn new<S: Into<String>>(deduction: u8, name: S) -> Detective {
        Detective {
            deduction,
            next: None,
            name: name.into(),
        }
    }
}

impl Policeman for Detective {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!(
                "Detective {}: I can't investigate it. I need help.",
                self.name
            );

            match self.next {
                Some(ref next) => next.investigate(crime),
                None => println!("Detective {}: Unimpossible for our department", self.name),
            }
        } else {
            println!("Detective {}: I can do this.", self.name);
        }
    }
}

pub struct Patrolman {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
    name: String,
}

impl Patrolman {
    pub fn new<S: Into<String>>(deduction: u8, name: S) -> Patrolman {
        Patrolman {
            deduction,
            next: None,
            name: name.into(),
        }
    }
}

impl Policeman for Patrolman {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!(
                "Patrolman {}: I'm just a patrolman. I need help.",
                self.name
            );

            match self.next {
                Some(ref next) => next.investigate(crime),
                None => println!("Patrolman {}: Unimpossible for our department.", self.name),
            }
        } else {
            println!("Patrolman {}: It's easy. I can do this.", self.name);
        }
    }
}

// Let's create a chain of policemen (Jack -> Tom -> Chuck).
// Every officer has an individual level of deduction.
// And every crime has a difficult lvl.
// Officer can investigate the crime by himself if his deduction is not less
// than crime's difficulty lvl.
// The officer passes the crime's case if the crime's difficulty lvl bigger
// than the officer's deduction.
fn main() {
    let chuck = Detective::new(8, "Chuck");

    let mut tom = Detective::new(5, "Tom");
    tom.set_next(Box::new(chuck));

    let mut jack = Patrolman::new(2, "Jack");
    jack.set_next(Box::new(tom));

    for crime_lvl in (1..=10u8).step_by(3) {
        println!("Investigation of a new crime (lvl: {crime_lvl})");
        jack.investigate(crime_lvl);
        println!();
    }
}

