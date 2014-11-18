#![feature(phase)]

#[phase(plugin)]
extern crate component_store;

#[deriving(Show, Clone)]
pub struct Position;

#[deriving(Show, Clone)]
pub struct Color;

component_store!(
  components:
    Color/Colorae <- Foo, Bar
    Position/Positionae <- Foo, Bar
)

fn main() {
  let ecs = ECS::new();
  ECS::color_foo();
  ECS::position_bar();
  println!("The result is {}", ecs);
}
