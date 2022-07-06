trait Subject {
    fn request(&self);
}

struct RealSubject;

struct Proxy<'a> {
    real_subject: &'a RealSubject
}

impl Proxy<'_> {
    fn new(rs: &RealSubject) -> Proxy {
        Proxy {
            real_subject: rs
        }
    }

    fn check_access() -> bool {
        println!("Proxy: Checking access prior to firing a real request.");
        true
    }

    fn log_access() -> bool {
        println!("Proxy: Logging the time of request.");
        true
    }
}

impl Subject for Proxy<'_> {
    fn request(&self) {
        if Proxy::check_access() {
            self.real_subject.request();
            Proxy::log_access();
        }
    }
}

impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling request.");
    }
}


fn main() {
    println!("Client: Executing the client code with a real subject:");
    let rs = RealSubject;
    rs.request();
    println!();

    println!("Client: Executing the same client code with a proxy:");
    let proxy = Proxy::new(&rs);
    proxy.request();
}
