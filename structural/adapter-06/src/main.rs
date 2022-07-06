
struct Target;

impl Target {
    fn request(&self) -> String {
        "Target: The default target\'s behavior.".to_string()
    }
}
struct Adaptee;

impl Adaptee {
    fn specific_request(&self) -> String {
        ".eetpadA eht fo roivaheb laicepS".to_string()
    }
}

struct Adapter<'a> {
    adaptee: &'a Adaptee
}

impl Adapter<'_> {
    fn new(apee: &Adaptee) -> Adapter {
        Adapter {
            adaptee: apee
        }
    }

    fn request(&self) -> String {
        let s: String = self.adaptee.specific_request().as_str().chars().rev().collect::<String>();
        format!("ADAPTER translated: {}", s)
    }
}

fn main() {
    let target = Target;
    println!("{}", target.request());

    let adaptee = Adaptee;
    println!("{}", adaptee.specific_request());

    let adapter = Adapter::new(&adaptee);
    println!("{}", adapter.request());

}
