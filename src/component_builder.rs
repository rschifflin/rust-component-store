use syntax::ast;
use syntax::ptr::P;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

#[deriving(Show, Clone)]
pub struct ComponentBuilder {
  pub name: String,
  pub plural: String,
  pub indices: Vec<String>
}

impl ComponentBuilder {
  fn lowerize(s: &String) -> String {
    let mut lower = String::new();
    for c in s.to_ascii().iter() {
      lower.push(c.to_lowercase().to_char());
    }
    lower
  }

  pub fn new(name: String, plural: String, indices: Vec<String>) -> ComponentBuilder {
    ComponentBuilder {
      name: name,
      plural: ComponentBuilder::lowerize(&plural),
      indices: indices
    }
  }

  pub fn build_decl(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let name_ident = ast::Ident::new(token::intern(self.name.as_slice()));
    let plural_ident = ast::Ident::new(token::intern(self.plural.as_slice()));

    quote_tokens!(context,
      pub $plural_ident: Vec<$name_ident>,
    )
  }

  pub fn build_init(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let name_ident = ast::Ident::new(token::intern(self.name.as_slice()));
    let plural_ident = ast::Ident::new(token::intern(self.plural.as_slice()));

    quote_tokens!(context,
      $plural_ident: Vec::new(),
    )
  }

  pub fn build_fns(&self, context: &ExtCtxt) -> Vec<P<ast::Item>> {
    let foo_string = ComponentBuilder::lowerize(&self.name) + "_foo".to_string();
    let bar_string = ComponentBuilder::lowerize(&self.name) + "_bar".to_string();

    let name_ident = ast::Ident::new(token::intern(self.name.as_slice()));
    let plural_ident = ast::Ident::new(token::intern(self.plural.as_slice()));
    let foo_ident = ast::Ident::new(token::intern(foo_string.as_slice()));
    let bar_ident = ast::Ident::new(token::intern(bar_string.as_slice()));

    let foo = quote_item!(context,
      pub fn $foo_ident() {
        println!("Foo");
      }
    );

    let bar = quote_item!(context,
      pub fn $bar_ident() {
        println!("Bar");
      }
    );

    let foobar = vec![foo, bar];
    foobar.into_iter().map(|item| item.unwrap()).collect()
  }
}

