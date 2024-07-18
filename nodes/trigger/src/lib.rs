wit_bindgen::generate!();

use exports::example::trigger::triggernode::Guest;

struct TriggerNode;

#[derive(Debug, serde::Serialize)]
struct Output {
    description: String,
}

impl Guest for TriggerNode {
    fn execute() -> String {

        let output_data = Output {
            description: String::from("Trigger is executed"),
        };

        let output = serde_json::to_value(&output_data).unwrap().to_string();
        return output;
    }
}

export!(TriggerNode);
