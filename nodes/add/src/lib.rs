wit_bindgen::generate!();

use exports::example::add::addnode::Guest;

struct AddNode;
#[derive(Debug, serde::Serialize)]
struct AddOutput {
    sum: i32,
    greeting: String,
}

impl Guest for AddNode {
    fn execute(params: String) -> String {
        let mut sum = 0;
        if let Ok(value) = serde_json::from_str::<Vec<i32>>(params.as_str()) {
            // Calculer la somme des nombres dans le vecteur
            let data_output = AddOutput {
                sum: value.iter().sum(),
                greeting: String::from("Thanks for your order"),
            };
            let output = serde_json::to_value(&data_output).unwrap().to_string();
            return output
        } else {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("Error while converting value in AddNode"),
            );
            // Retourner une erreur ou une valeur par défaut si le paramètre n'est pas trouvé
            return "Numbers to add not found".to_string();
        }

        format!("{sum}").to_string()
    }
}

export!(AddNode);
