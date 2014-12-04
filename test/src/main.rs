#![feature(phase)]

#[phase(plugin)]
extern crate component_store;

use std::collections::HashMap;
#[deriving(Show, Clone)]
pub struct Position;

#[deriving(Show, Clone)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8
}

#[deriving(Show, Clone)]
pub struct Entry {
  data: uint
}

component_store!(
  components:
    Color
    Position/Positions <- Range
    Entry/Entries
)

fn main() {
  let mut ecs = ECS::new();
  ecs.colors.update_color("white", Color { r: 255, g: 255, b: 255 });
  ecs.colors.remove_all_colors();

  ecs.colors.update_color("red", Color { r: 255, g: 0, b: 0 });
  ecs.colors.update_color("blue", Color { r: 0, g: 0, b: 255 });
  ecs.colors.update_color("red", Color { r: 128, g: 0, b: 0 });

  ecs.entries.update_entry("e1", Entry { data: 12340912 });
  ecs.entries.update_entry("e2", Entry { data: 20958129 });
  ecs.entries.remove_entry("e2");

  println!("Colors: {}", ecs.colors.find_all_colors());
  println!("Entries: {}", ecs.entries.find_all_entries());
}
