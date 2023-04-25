use std::cell::RefCell;

use magnus::{define_module, function, prelude::*, value::Value,
             Error, class, method, Object, block::{block_given, Yield}, IntoValue};

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

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
#[magnus::wrap(class = "AluminiumOxide::Capitalizer", mark, size)]
pub struct Capitalizer {
    name: RefCell<String>
}

impl Capitalizer {
    fn new(name: String) -> Self {
        Self {name: RefCell::new(name)}
    }
    fn capitalize(&self) -> Yield<impl Iterator<Item = char>>{
        if block_given() {
            Yield::Iter(self.name.borrow().chars().map(|c| c.to_ascii_uppercase())
                            .collect::<Vec<char>>().into_iter())
        } else {
            Yield::Enumerator(self.clone().into_value().enumeratorize("capitalize", ()))
        }
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("AluminiumOxide")?;

    module.define_singleton_method("hello", function!(hello, 1))?;

    let rb_class = module.define_class("Greeter", class::object())?;
    rb_class.define_singleton_method("new", function!(Greeter::new, 1))?;
    rb_class.define_method("greet", method!(Greeter::greet, 0))?;

    let cap_class = module.define_class("Capitalizer", class::object())?;
    cap_class.define_singleton_method("new", function!(Capitalizer::new, 1))?;
    cap_class.define_method("capitalize", method!(Capitalizer::capitalize, 0))?;
    
    let g = Greeter{name: String::from("From Rust")};
    module.const_set("GREETER", g)?;

    Ok(())
}
