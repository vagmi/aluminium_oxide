# AluminiumOxide

This gem explores the patterns to integrate Ruby with Rust.

* [x] Create a ruby gem that has a rust extension
* [x] Setup Github actions
* [x] Call Rust function from Ruby exposed in a module
* [x] Create a Ruby Class in the Rust extension
* [x] Yield to a block from Rust code
* [ ] Call a async rust method and provide have the rust method yield to a block
* [ ] Call ruby code from Rust


### :hammer: Building and Running

Ensure that you have Ruby and Rustup installed. You can then do the following.

```
$ bundle exec rake compile
$ bundle exec irb
> require 'aluminium_oxide'
AluminiumOxide.hello 'world'
=> "Hello from Rust, world"
```

