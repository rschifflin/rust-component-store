use syntax::parse::parser::Parser;
use syntax::parse::token;
use ecs_builder::ECSBuilder;
use component_builder::ComponentBuilder;
use utils::result_utils::ResultUtils;

pub fn parse(parser: &mut Parser) -> Result<ECSBuilder, &'static str> {

  match parse_component_header(parser) {
    Ok(_) =>
      parse_components(parser).map(|component_builders| -> ECSBuilder {
        ECSBuilder {
          component_builders: component_builders
        }
      }),
    Err(e) => Err(e)
  }
}

fn parse_component_header(parser: &mut Parser) -> Result<(), &'static str> {
  match parse_ident(parser) {
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
  parse_components_recursive(parser, Vec::new())
}

fn parse_components_recursive(parser: &mut Parser, mut components: Vec<Result<ComponentBuilder, &'static str>>) -> Result<Vec<ComponentBuilder>, &'static str> {
  components.push(parse_component(parser));
  match parser.token {
    token::Ident(_, _) => { parse_components_recursive(parser, components) }
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
  parse_component_name(parser).flat_map(|name| -> Result<ComponentBuilder, &'static str> {
    parse_optional_plural(parser).flat_map(|plural| -> Result<ComponentBuilder, &'static str> {
      parse_optional_indices(parser).flat_map(|indices| -> Result<ComponentBuilder, &'static str> {
        let plural_or_default = plural.clone().unwrap_or(name + "s".to_string());
        let indices_or_default = indices.clone().unwrap_or(Vec::new());
        Ok(ComponentBuilder::new(name.clone(), plural_or_default, indices_or_default))
      })
    })
  })
}

fn parse_component_name(parser: &mut Parser) -> Result<String, &'static str> {
  match parse_ident(parser) {
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
    token::Ident(i, _) => Ok(Some(i.as_str().to_string())),
    _ => Err("Pluralization name not found after slash")
  };
  parser.bump();
  result
}

fn parse_optional_indices(parser: &mut Parser) -> Result<Option<Vec<String>>, &'static str> {
  if !parser.eat(&token::LArrow) { return Ok(None) };
  parse_index_list(parser).map(|inner| -> Option<Vec<String>> { Some(inner) })
}

fn parse_index_list(parser: &mut Parser) -> Result<Vec<String>, &'static str> {
  parse_index_list_recursive(parser, Vec::new())
}

fn parse_index_list_recursive(parser: &mut Parser, mut indices: Vec<Result<String, &'static str>>) -> Result<Vec<String>, &'static str> {
  indices.push(parse_index(parser));
  if parser.eat(&token::Comma) { parse_index_list_recursive(parser, indices) }
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
  match parse_ident(parser) {
    Ok(pass) => Ok(pass),
    Err(_) => Err("Failed to parse index name")
  }
}

