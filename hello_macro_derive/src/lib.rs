extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait(?Send)]
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }

            fn from_node(node: Node) -> Option<Entity> {
                Some(
                        Entity::#name(#name {
                            title : String::from("title"),
                            tagline : String::from("tagline"),
                            released : 1992
                        })
                    )
            }

            async fn find_one(conn: &Connection) -> Result<Entity, Box<dyn std::error::Error>> {
                let pull_meta = Metadata::from_iter(vec![("n", 1)]);
                // todo: figure out how to generate this query string
                let (_response, records) = conn.run("MATCH (n:Movie) RETURN n;", pull_meta).await?;
                let node = Node::try_from(records[0].fields()[0].clone())?;
                Ok(#name::from_node(node).unwrap())
            }
        }
    };
    
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
