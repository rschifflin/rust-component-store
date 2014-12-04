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
  pub find: ast::Ident,
  pub find_all: ast::Ident,
  pub remove: ast::Ident,
  pub remove_all: ast::Ident,
  pub update: ast::Ident
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
        index: ast::Ident::new(token::intern((name + "Index").as_slice())),
        find: ast::Ident::new(token::intern(("find_".to_string() + lower_case(&name)).as_slice())),
        find_all: ast::Ident::new(token::intern(("find_all_".to_string() + plural).as_slice())),
        remove: ast::Ident::new(token::intern(("remove_".to_string() + lower_case(&name)).as_slice())),
        remove_all: ast::Ident::new(token::intern(("remove_all_".to_string() + plural).as_slice())),
        update: ast::Ident::new(token::intern(("update_".to_string() + lower_case(&name)).as_slice()))
      }
    }
  }

  pub fn build_index(&self, context: &ExtCtxt) -> Vec<Option<P<ast::Item>>> {
    let name_ident = self.idents.name.clone();
    let index_ident = self.idents.index.clone();
    let find_ident = self.idents.find.clone();
    let find_all_ident = self.idents.find_all.clone();
    let remove_ident = self.idents.remove.clone();
    let remove_all_ident = self.idents.remove_all.clone();
    let update_ident = self.idents.update.clone();

    let structure = quote_item!(context,
      #[deriving(Clone, Show)]
      pub struct $index_ident<'a> {
        primary_index: HashMap<&'a str, $name_ident>
      }
    );

    let implementation = quote_item!(context,
      impl<'a> $index_ident<'a> {
        pub fn new() -> $index_ident<'a> {
          $index_ident {
            primary_index: HashMap::new()
          }
        }

        pub fn $find_all_ident(&self) -> Vec<&$name_ident> {
          self.primary_index.values().collect()
        }

        pub fn $find_ident(&self, key: &'a str) -> Option<&$name_ident> {
          self.primary_index.get(&key)
        }

        pub fn $update_ident(&mut self, key: &'a str, value: $name_ident) -> Option<$name_ident> {
          self.primary_index.insert(key, value)
        }

        pub fn $remove_all_ident(&mut self) {
          self.primary_index = HashMap::new();
        }

        pub fn $remove_ident(&mut self, key: &'a str) {
          self.primary_index.remove(&key);
        }
      }
    );

    vec!(structure, implementation)
  }

  pub fn build_decl(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.clone();
    let plural_ident = self.idents.plural.clone();

    quote_tokens!(context,
      pub $plural_ident: $index_ident<'a>,
    )
  }

  pub fn build_init(&self, context: &ExtCtxt) -> Vec<ast::TokenTree> {
    let index_ident = self.idents.index.clone();
    let plural_ident = self.idents.plural.clone();

    quote_tokens!(context,
      $plural_ident: $index_ident::new(),
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
