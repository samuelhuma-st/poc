wit_bindgen::generate!();

use exports::example::print::printnode::Guest;

struct PrintNode;

impl Guest for PrintNode {
    fn execute(params: String) -> String {
        // Retourner le résultat sous forme de boîte de Debug pour l'affichage
        wasi::logging::logging::log(
            wasi::logging::logging::Level::Info,
            "",
            &format!("Output : {params:?}"),
        );

        "Executed successfully".to_string()
    }
}

export!(PrintNode);
