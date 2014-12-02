use syntax::ast;
use syntax::ptr::P;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

use utils::string_utils::lower_case;

#[deriving(Show, Clone)]
pub struct ComponentBuilder {
  pub name: String,
  pub plural: String,
  pub indices: Vec<String>,

  pub idents: ComponentBuilderIdents
}

#[deriving(Show, Clone)]
pub struct ComponentBuilderIdents {
  pub name: ast::Ident,
  pub plural: ast::Ident,
  pub index: ast::Ident,
}

impl ComponentBuilder {
  pub fn new(name: String, plural: String, indices: Vec<String>) -> ComponentBuilder {
    ComponentBuilder {
      name: name.clone(),
      plural: lower_case(&plural),
      indices: indices,
      idents: ComponentBuilderIdents {
        name: ast::Ident::new(token::intern(name.as_slice())),
        plural: ast::Ident::new(token::intern(plural.as_slice())),
        index: ast::Ident::new(token::intern((name + "Index").as_slice()))
      }
    }
  }

  pub fn build_index(&self, context: &ExtCtxt) -> Option<P<ast::Item>> {
    let name_ident = self.idents.name.clone();
    let index_ident = self.idents.index.clone();

    quote_item!(context,
      #[deriving(Show, Clone)]
      pub struct $index_ident;
    )
  }

  pub fn build_decl(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.clone();
    let plural_ident = self.idents.plural.clone();

    quote_tokens!(context,
      pub $plural_ident: $index_ident,
    )
  }

  pub fn build_init(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.clone();
    let plural_ident = self.idents.plural.clone();

    quote_tokens!(context,
      $plural_ident: $index_ident,
    )
  }

  pub fn build_fns(&self, context: &ExtCtxt) -> Vec<P<ast::Item>> {
    let foo_string = lower_case(&self.name) + "_foo".to_string();
    let bar_string = lower_case(&self.name) + "_bar".to_string();

    let name_ident = self.idents.name.clone();
    let plural_ident = self.idents.plural.clone();
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
