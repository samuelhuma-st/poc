wit_bindgen::generate!();

use exports::example::add::addnode::Guest;

struct AddNode;
#[derive(Debug, serde::Serialize)]
struct Output {
    result: i32,
    description: String,
}

impl Guest for AddNode {
    fn execute(params: String) -> String {
        if let Ok(value) = serde_json::from_str::<Vec<i32>>(params.as_str()) {
            let data_output = Output {
                result: value.iter().sum(),
                description: String::from("This node performs an addition"),
            };

            let output = serde_json::to_value(&data_output).unwrap().to_string();
            return output;
        } else {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("Error while converting value in AddNode"),
            );

            return "Numbers to add not found".to_string();
        }
    }
}

export!(AddNode);
