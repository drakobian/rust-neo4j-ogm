extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Queryable)]
pub fn queryable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_queryable(&ast)
}

fn impl_queryable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait(?Send)]
        impl Queryable for #name {
            fn from_node(node: Node) -> Option<Entity> {
                Some(
                        Entity::#name(#name {
                            title : String::from("Gone With the Wind"),
                            tagline : String::from("This time......it's personal"),
                            released : 1992
                        })
                    )
            }

            async fn find(conn: &Connection, n: i32) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
                let pull_meta = Metadata::from_iter(vec![("n", n)]);
                // todo: figure out how to generate this query string
                //let query = format!("MATCH (n:{}) RETURN N;", stringify!(#name).to_string()).to_string();
                let query = format!("MATCH (n:{}) RETURN n;", stringify!(#name));  
                let (_response, records) = conn.run(&query, pull_meta).await?;
                let mut vec = Vec::new();
                for record in records {
                    let node = Node::try_from(record.fields()[0].clone())?;
                    vec.push(#name::from_node(node).unwrap());
                }
                Ok(vec)
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
