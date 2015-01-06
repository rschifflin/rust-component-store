#![feature(phase)]

#[phase(plugin)]
extern crate component_store;

use std::collections::HashMap;
#[derive(Show, Clone)]
pub struct PositionComponent;

#[derive(Show, Clone)]
pub struct ColorComponent {
  r: u8,
  g: u8,
  b: u8
}

#[derive(Show, Clone)]
pub struct EntryComponent {
  data: uint
}

component_store!(
  components:
    Color
    Position/Positions <- Range
    Entry/Entries
);

fn main() {
  let mut ecs = ECS::new();
  ecs.colors.update_color("white".to_string(), ColorComponent { r: 255, g: 255, b: 255 });
  ecs.colors.remove_all_colors();

  ecs.colors.update_color("red".to_string(), ColorComponent { r: 255, g: 0, b: 0 });
  ecs.colors.update_color("blue".to_string(), ColorComponent { r: 0, g: 0, b: 255 });
  ecs.colors.update_color("red".to_string(), ColorComponent { r: 128, g: 0, b: 0 });

  ecs.entries.update_entry("e1".to_string(), EntryComponent { data: 12340912 });
  ecs.entries.update_entry("e2".to_string(), EntryComponent { data: 20958129 });
  ecs.entries.remove_entry(&"e2".to_string());

  println!("Colors: {}", ecs.colors.find_all_colors());
  println!("Entries: {}", ecs.entries.find_all_entries());
}
