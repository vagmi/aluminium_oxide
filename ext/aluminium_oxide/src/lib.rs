use magnus::{define_module, function, prelude::*, 
             Error, class, method, Object};

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}


#[magnus::wrap(class = "AluminiumOxide::Greeter")]
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Greeter {
    name: String
}

impl Greeter {
    fn new(name: String) -> Self {
        Self {name}
    }

    fn greet(&self) -> String {
        format!("Hello, {}!", &self.name)
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("AluminiumOxide")?;

    module.define_singleton_method("hello", function!(hello, 1))?;

    let rb_class = module.define_class("Greeter", class::object())?;
    rb_class.define_singleton_method("new", function!(Greeter::new, 1))?;
    rb_class.define_method("greet", method!(Greeter::greet, 0))?;
    
    let g = Greeter{name: String::from("From Rust")};
    module.const_set("GREETER", g)?;

    Ok(())
}
