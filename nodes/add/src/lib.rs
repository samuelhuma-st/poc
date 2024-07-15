wit_bindgen::generate!();

use exports::example::add::addnode::Guest;

struct AddNode;

impl Guest for AddNode {
    fn execute(params: String) -> String {
        let mut sum = 0;
        if let Ok(value) = serde_json::from_str::<Vec<i32>>(params.as_str()) {
            // Calculer la somme des nombres dans le vecteur
            sum = value.iter().sum();
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
