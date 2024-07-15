wit_bindgen::generate!();

use exports::example::add::addnode::Guest;

struct AddNode;

impl Guest for AddNode {
    fn execute() -> String {
        String::from("user")
    }
}

export!(AddNode);
