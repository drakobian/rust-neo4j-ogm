use bolt_client::*;
use bolt_proto::{value::*};

use ogm::{models::Movie};

pub trait HelloMacro {
    fn hello_macro();
    fn test_import_stuff(node: Node) -> Option<Movie>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
