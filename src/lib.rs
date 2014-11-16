#![crate_type="dylib"]
#![feature(plugin_registrar)]

extern crate syntax;
extern crate rustc;

use syntax::parse::token;
use syntax::parse;
use syntax::parse::parser::Parser;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacExpr, MacResult};
use syntax::codemap::Span;
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  reg.register_macro("component_store", expand);
}

#[deriving(Show, Clone)]
pub struct ComponentBuilder {
  name: String,
  plural: String,
  indices: Vec<String>
}

pub struct ECSParser;

#[deriving(Show)]
pub struct ECSBuilder {
  pub component_builders: Vec<ComponentBuilder>
}

impl ECSParser {
  pub fn parse(parser: &mut Parser) -> Result<ECSBuilder, &'static str> {
    match ECSParser::parse_component_header(parser) {
      Ok(_) =>
        ECSParser::parse_components(parser).map(|component_builders| -> ECSBuilder {
          ECSBuilder {
            component_builders: component_builders
          }
        }),
      Err(e) => Err(e)
    }
  }

  fn parse_component_header(parser: &mut Parser) -> Result<(), &'static str> {
    match ECSParser::parse_ident(parser) {
      Ok(parsed_ident) =>{
        if parsed_ident.as_slice() == "components" {
          if parser.eat(&token::Colon) { Ok(()) }
          else { Err("Expected header to be followed by a colon") }
        }
        else { Err("Expected header to be `components:`") }
      },
      Err(e) => Err(e)
    }
  }

  fn parse_components(parser: &mut Parser) -> Result<Vec<ComponentBuilder>, &'static str> {
    ECSParser::parse_components_recursive(parser, Vec::new())
  }

  fn parse_components_recursive(parser: &mut Parser, mut components: Vec<Result<ComponentBuilder, &'static str>>) -> Result<Vec<ComponentBuilder>, &'static str> {
    components.push(ECSParser::parse_component(parser));
    match parser.token {
      token::Ident(_, _) => { ECSParser::parse_components_recursive(parser, components) }
      token::Eof => {
        components.iter().fold(Ok(Vec::new()), |z, elem| -> Result<Vec<ComponentBuilder>, &'static str> {
          let copy_elem = elem.clone();
          match copy_elem {
            Err(e) => Err(e),
            Ok(component) => z.map(|v| -> Vec<ComponentBuilder> {
              let mut next_vec = v.clone();
              next_vec.push(component.clone());
              next_vec
            })
          }
        })
      }
      _ => { Err("Failed to parse list of components") }
    }
  }

  fn parse_component(parser: &mut Parser) -> Result<ComponentBuilder, &'static str> {
    //TODO "Use flat_map instead of early return try!s"

    let name = try!(ECSParser::parse_component_name(parser));
    let plural = try!(ECSParser::parse_optional_plural(parser));
    let indices = try!(ECSParser::parse_optional_indices(parser));

    let plural_or_default = plural.unwrap_or(name + "s".to_string());
    let indices_or_default = indices.unwrap_or(Vec::new());

    Ok(ComponentBuilder {
      name: name,
      plural: plural_or_default,
      indices: indices_or_default
    })
  }

  fn parse_component_name(parser: &mut Parser) -> Result<String, &'static str> {
    match ECSParser::parse_ident(parser) {
      Ok(pass) => Ok(pass),
      Err(_) => Err("Failed to parse component name")
    }
  }

  fn parse_ident(parser: &mut Parser) -> Result<String, &'static str> {
    let result = match parser.token {
      token::Ident(i, _) => Ok(i.as_str().to_string()),
      _ => Err("Failed to parse ident")
    };
    parser.bump();
    result
  }

  fn parse_optional_plural(parser: &mut Parser) -> Result<Option<String>, &'static str> {
    if !parser.eat(&token::BinOp(token::Slash)) { return Ok(None) };

    let result = match parser.token {
      token::Ident(i, _) => Ok(Some(i.to_string())),
      _ => Err("Pluralization name not found after slash")
    };
    parser.bump();
    result
  }

  fn parse_optional_indices(parser: &mut Parser) -> Result<Option<Vec<String>>, &'static str> {
    if !parser.eat(&token::LArrow) { return Ok(None) };
    ECSParser::parse_index_list(parser).map(|inner| -> Option<Vec<String>> { Some(inner) })
  }

  fn parse_index_list(parser: &mut Parser) -> Result<Vec<String>, &'static str> {
    ECSParser::parse_index_list_recursive(parser, Vec::new())
  }

  fn parse_index_list_recursive(parser: &mut Parser, mut indices: Vec<Result<String, &'static str>>) -> Result<Vec<String>, &'static str> {
    indices.push(ECSParser::parse_index(parser));
    if parser.eat(&token::Comma) { ECSParser::parse_index_list_recursive(parser, indices) }
    else {
      indices.iter().fold(Ok(Vec::new()), |z, elem| -> Result<Vec<String>, &'static str> {
        let copy_elem = elem.clone();
        match copy_elem {
          Err(e) => Err(e),
          Ok(index) => z.map(|v| -> Vec<String> {
            let mut next_vec = v.clone();
            next_vec.push(index.clone());
            next_vec
          })
        }
      })
    }
  }

  fn parse_index(parser: &mut Parser) -> Result<String, &'static str> {
    match ECSParser::parse_ident(parser) {
      Ok(pass) => Ok(pass),
      Err(_) => Err("Failed to parse index name")
    }
  }
}

fn expand(context: &mut ExtCtxt, span: Span, tokens: &[TokenTree]) -> Box<MacResult + 'static> {
  let result_string = match parse(context, tokens) {
    Ok(result) => result.to_string(),
    Err(e) => e.to_string()
  };
  let result_interned = token::intern_and_get_ident(result_string.as_slice());
  MacExpr::new(context.expr_str(span, result_interned))
}

fn parse(context: &mut ExtCtxt, tokens: &[TokenTree]) -> Result<ECSBuilder, &'static str> {
  let mut parser = parse::new_parser_from_tts(context.parse_sess(), context.cfg(), tokens.to_vec());
  ECSParser::parse(&mut parser)
}
