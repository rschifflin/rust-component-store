#![feature(phase)]

#[phase(plugin)]
extern crate component_store;

#[deriving(Show, Clone)]
pub struct Position;

#[deriving(Show, Clone)]
pub struct Color;

#[deriving(Show, Clone)]
pub struct Radius;

component_store!(
  components:
    Color
    Position/Positions <- Range
)

fn main() {
  let ecs = ECS::new();
  ECS::color_foo();
  ECS::position_bar();
  println!("The result is {}", ecs);
}
