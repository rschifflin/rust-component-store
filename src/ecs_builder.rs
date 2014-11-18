use syntax::ast;
use syntax::ptr::P;
use component_builder::ComponentBuilder;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

#[deriving(Show)]
pub struct ECSBuilder {
  pub component_builders: Vec<ComponentBuilder>
}

impl ECSBuilder {
  pub fn build(&self, context: &ExtCtxt) -> Vec<P<ast::Item>> {
    let component_decls: Vec<Vec<ast::TokenTree>> =
      self.component_builders.iter().map(|builder| -> Vec<ast::TokenTree> {
        builder.build_decl(context)
      }).collect();

    let component_inits: Vec<Vec<ast::TokenTree>> =
      self.component_builders.iter().map(|builder| -> Vec<ast::TokenTree> {
        builder.build_init(context)
      }).collect();

    let component_fns: Vec<Vec<P<ast::Item>>> =
      self.component_builders.iter().map(|builder| -> Vec<P<ast::Item>> {
        builder.build_fns(context)
      }).collect();

    let structure = quote_item!(context,
      #[deriving(Show, Clone)]
      pub struct ECS {
        $component_decls
      };
    );

    let implementation = quote_item!(context,
      impl ECS {
        pub fn new() -> ECS {
          ECS {
            $component_inits
          }
        }

        $component_fns
      }
    );

    let items = vec!(structure, implementation);
    items.into_iter().map(|item| item.unwrap()).collect()
  }
}
