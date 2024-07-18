use std::collections::HashMap;

use serde_json::Value;

use crate::utils::resolve_references;
use crate::utils::NodeFunction;
use crate::{
    models::WorkflowData,
    utils::{build_dependency_graph, topological_sort},
    wasi,
};
pub struct WorkflowRunner {}

impl WorkflowRunner {
    pub fn run(workflow_data: &WorkflowData, all_nodes: HashMap<&str, NodeFunction>) {
        let mut execution_results: HashMap<String, Value> = HashMap::new();

        let graph = build_dependency_graph(workflow_data);
        let sorted_nodes = topological_sort(&workflow_data, &graph);

        let trigger_node = workflow_data
            .nodes
            .iter()
            .find(|&n| n.node_type == "trigger");

        if let None = trigger_node {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Error,
                "",
                &format!("Aucun nœud de type 'trigger' trouvé."),
            );

            return;
        }

        if sorted_nodes.is_empty() {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Error,
                "",
                &format!("L'ordre des nœuds est incorrect."),
            );

            return;
        }

        // Vérifier si le nœud de départ existe
        if !sorted_nodes.contains(&trigger_node.unwrap().id) {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Error,
                "",
                &format!("Le nœud de départ n'existe pas dans l'ordre trié."),
            );

            return;
        }

        // Exécution des nœuds dans l'ordre topologique à partir du nœud de départ
        let start_index = sorted_nodes
            .iter()
            .position(|id| *id == trigger_node.unwrap().id)
            .unwrap();

        for node_id in &sorted_nodes[start_index..] {
            if let Some(current_node) = workflow_data.nodes.iter().find(|&n| n.id == *node_id) {
                let a = current_node.node_type.as_str();
                if let Some(node_box) = all_nodes.get(&a) {
                    // Résoudre les références dans les paramètres
                    let resolved_params = resolve_references(
                        &current_node.parameters.clone().unwrap(),
                        &execution_results,
                    );

                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("input_data = {resolved_params:?}"),
                    );

                    let value = match resolved_params.get("value") {
                        Some(x) => x,
                        None => &"".to_string(),
                    };

                    let mut result = String::new();

                    match node_box {
                        NodeFunction::NoParam(f) => {
                            result = f();
                        }
                        NodeFunction::WithParam(f) => {
                            result = f(value);
                        }
                    }

                    let result_format = result.to_string();
                    let output_data: Value = serde_json::from_str(&result_format).unwrap();

                    execution_results.insert(
                        current_node.name.clone(),
                        serde_json::from_str(format!("{{\"json\": {output_data}}}").as_str()).unwrap(),
                    );

                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("execution_results = {execution_results:?}"),
                    );
                } else {
                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("Node type {} not found in all_nodes", current_node.name),
                    );
                }
            }
        }
    }
}
