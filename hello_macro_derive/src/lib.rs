extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn;
use syn::{Data, Fields};

use syn::spanned::Spanned;
#[proc_macro_derive(Queryable)]
pub fn queryable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_queryable(&ast)
}


fn impl_queryable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_fields: TokenStream = 
    //fn get_fields(data: &syn::Data) {
        //match *data {
        match ast.data {
            Data::Struct(ref data) => {
                match data.fields {
                    Fields::Named(ref fields) => {
                        let recurse = fields.named.iter().map(|f| {
                            let name = f.ident.as_ref().unwrap();
                            let db_name = format!("{}", name);
                            let typ = &f.ty;
                            quote! {
                                #name : #typ::try_from(node.properties().get(#db_name).unwrap().clone()).ok()?,
                            }
                        });
                        let gen = quote! {
                            #(#recurse)*
                        };
                        gen.into()
                    }
                    Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
                }
            }
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
        };
    //}
    //let struct_fields = get_fields(&ast.data);
    println!("{}", struct_fields);

    let gen = quote! {
        #[async_trait(?Send)]
        impl Queryable for #name {
            fn from_node(node: Node) -> Option<Entity> {
                Some(Entity::#name(#name { }))
            }

            async fn find(conn: &Connection, n: i32) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
                let pull_meta = Metadata::from_iter(vec![("n", n)]); 
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
