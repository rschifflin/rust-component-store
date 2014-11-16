#![feature(phase)]

#[phase(plugin)]
extern crate component_store;

fn main() {
  println!("The result is {}", component_store!(
             components:
              Position <- Test, More
              Color <- Foo, Bar
            )
  );
}

