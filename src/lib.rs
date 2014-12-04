#![crate_type="dylib"]
#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;

use syntax::ast;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use syntax::parse;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::codemap::Span;
use rustc::plugin::Registry;
use ecs_builder::ECSBuilder;

mod utils {
  pub mod string_utils;
  pub mod result_utils;
}

mod ecs_parser;
mod ecs_builder;
mod component_builder;


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  reg.register_macro("component_store", expand);
}

fn expand(context: &mut ExtCtxt, _: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
  let ecs_builder = match parse(context, tokens) {
    Ok(result) => result,
    Err(e) => panic!(e)
  };
  let ecs = MacroResult { ecs: ecs_builder.build(context) };
  box ecs as Box<MacResult>
}

struct MacroResult {
  ecs: Vec<P<ast::Item>>
}

impl MacResult for MacroResult {
  fn make_items(self: Box<MacroResult>) -> Option<SmallVector<P<ast::Item>>> {
    Some(SmallVector::many(self.ecs.clone()))
  }
}

fn parse(context: &mut ExtCtxt, tokens: &[TokenTree]) -> Result<ECSBuilder, &'static str> {
  let mut parser = parse::new_parser_from_tts(context.parse_sess(), context.cfg(), tokens.to_vec());
  ecs_parser::parse(&mut parser)
}
