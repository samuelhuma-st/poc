wit_bindgen::generate!();

use exports::example::trigger::triggernode::Guest;

struct TriggerNode;

impl Guest for TriggerNode {
    fn execute() -> String {
        format!("Trigger is executed")
    }
}

export!(TriggerNode);
