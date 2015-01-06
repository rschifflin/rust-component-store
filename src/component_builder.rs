use syntax::ast;
use syntax::ptr::P;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;
use utils::string_utils::snake_case;

#[derive(Show, Clone)]
pub struct IdentPair {
  snake: ast::Ident,
  camel: ast::Ident
}

impl IdentPair {
  pub fn new(name: &String) -> IdentPair {
    IdentPair {
      snake: ast::Ident::new(token::intern(snake_case(name).as_slice())),
      camel: ast::Ident::new(token::intern(name.as_slice())),
    }
  }
}

#[derive(Show, Clone)]
pub struct ComponentBuilder {
  pub name: String,
  pub plural: String,
  pub indices: Vec<String>,

  pub idents: ComponentBuilderIdents
}

#[derive(Show, Clone)]
pub struct ComponentBuilderIdents {
  pub name: IdentPair,
  pub plural: IdentPair,
  pub index: IdentPair,
  pub find: IdentPair,
  pub find_all: IdentPair,
  pub remove: IdentPair,
  pub remove_all: IdentPair,
  pub update: IdentPair
}

impl ComponentBuilder {
  pub fn new(name: String, plural: String, indices: Vec<String>) -> ComponentBuilder {
    ComponentBuilder {
      name: name.clone(),
      plural: plural.clone(),
      indices: indices,
      idents: ComponentBuilderIdents {
        name: IdentPair::new(&(name.clone() + "Component")),
        plural: IdentPair::new(&plural),
        index: IdentPair::new(&(name.clone() + "Index")),
        find: IdentPair::new(&("find_".to_string() + name.as_slice())),
        find_all: IdentPair::new(&("find_all_".to_string() + plural.as_slice())),
        remove: IdentPair::new(&("remove_".to_string() + name.as_slice())),
        remove_all: IdentPair::new(&("remove_all_".to_string() + plural.as_slice())),
        update: IdentPair::new(&("update_".to_string() + name.as_slice()))
      }
    }
  }

  pub fn build_index(&self, context: &ExtCtxt) -> Vec<Option<P<ast::Item>>> {
    let name_ident = self.idents.name.camel.clone();
    let index_ident = self.idents.index.camel.clone();
    let find_ident = self.idents.find.snake.clone();
    let find_all_ident = self.idents.find_all.snake.clone();
    let remove_ident = self.idents.remove.snake.clone();
    let remove_all_ident = self.idents.remove_all.snake.clone();
    let update_ident = self.idents.update.snake.clone();

    let structure = quote_item!(context,
      #[derive(Clone, Show)]
      pub struct $index_ident {
        primary_index: HashMap<String, $name_ident>
      }
    );

    let implementation = quote_item!(context,
      impl $index_ident {
        pub fn new() -> $index_ident {
          $index_ident {
            primary_index: HashMap::new()
          }
        }

        pub fn $find_all_ident(&self) -> Vec<&$name_ident> {
          self.primary_index.values().collect()
        }

        pub fn $find_ident(&self, key: String) -> Option<&$name_ident> {
          self.primary_index.get(&key)
        }

        pub fn $update_ident(&mut self, key: String, value: $name_ident) -> Option<$name_ident> {
          self.primary_index.insert(key, value)
        }

        pub fn $remove_all_ident(&mut self) {
          self.primary_index = HashMap::new();
        }

        pub fn $remove_ident(&mut self, key: String) {
          self.primary_index.remove(&key);
        }
      }
    );

    vec!(structure, implementation)
  }

  pub fn build_decl(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.camel.clone();
    let plural_ident = self.idents.plural.snake.clone();

    quote_tokens!(context,
      pub $plural_ident: $index_ident,
    )
  }

  pub fn build_init(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.camel.clone();
    let plural_ident = self.idents.plural.snake.clone();

    quote_tokens!(context,
      $plural_ident: $index_ident::new(),
    )
  }
}
