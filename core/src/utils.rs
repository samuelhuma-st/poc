use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, error::Error};

use crate::{models::WorkflowData, wasi};

pub enum NodeFunction {
    NoParam(fn() -> String),
    WithParam(fn(&str) -> String),
}

pub fn parse_workflow_data(body: &str) -> Result<WorkflowData, Box<dyn Error>> {
    let workflow_data: WorkflowData = serde_json::from_str(&body)?;
    Ok(workflow_data)
}

pub fn resolve_references(
    params: &HashMap<String, Value>,
    node_results: &HashMap<String, Value>,
) -> HashMap<String, String> {
    params
        .iter()
        .map(|(k, v)| {
            let mut value = String::new();

            if node_results.is_empty() {
                return (k.clone(), v.clone().to_string());
            }

            for (node_id, result) in node_results {
                value = replace_placeholders(v.to_string().as_str(), node_results);
                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("here is value = {value:?}"),
                );
            }

            (k.clone(), value)
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    greeting: String,
    sum: u32,
}
// To replace a placeholder by it's value
pub fn replace_placeholders(input: &str, map: &HashMap<String, Value>) -> String {
    let mut result = String::new();
    let mut i = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while i < input_chars.len() {
        if input_chars[i] == '{' && i + 3 < input_chars.len() && &input[i..i + 4] == "{{$(" {
            if let Some(end_idx) = input[i..].find("}}") {
                let placeholder = &input[i + 4..i + end_idx];
                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("placeholder = {placeholder}"),
                );
                let splitted_str: (&str, &str) = placeholder.split_once(").").unwrap();
                let trimmed = splitted_str.0.trim_matches(&['\"', '\\']);

                let value_path: Vec<&str> = splitted_str.1.split('.').collect();

                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("trimmed = {trimmed}"),
                );

                let x = &map.get(trimmed).unwrap();
                if let Some(val) = get_value_from_path(x, &value_path) {
                    result.push_str(&val.to_string());
                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("result 👍= {val:?}"),
                    );
                }

                i += end_idx + 2; // Move past the end of the placeholder
            } else {
                result.push(input_chars[i]);
                i += 1;
            }
        } else {
            result.push(input_chars[i]);
            i += 1;
        }
    }

    result
}

fn get_value_from_path<'a>(map: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current_value: Option<&Value> = None;

    for (i, key) in path.iter().enumerate() {
        if i == 0 {
            current_value = map.get(*key);
        } else if let Some(val) = current_value {
            current_value = val.get(*key);
        } else {
            return None;
        }
    }

    current_value
}

// Fonction pour construire le graphe des dépendances
pub fn build_dependency_graph(workflow_data: &WorkflowData) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for node_data in &workflow_data.nodes {
        if let Some(next_node) = node_data.clone().next_node {
            graph
                .entry(node_data.id.clone())
                .or_default()
                .push(next_node);
        }
    }
    graph
}

// Fonction pour effectuer un tri topologique
pub fn topological_sort(
    workflow_data: &WorkflowData,
    graph: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut in_degree = HashMap::new();
    for node_data in workflow_data.nodes.clone() {
        in_degree.insert(node_data.id, 0);
    }

    for nodes in graph.values() {
        for node_id in nodes.clone() {
            *in_degree.entry(node_id).or_default() += 1;
        }
    }

    let mut stack: Vec<String> = Vec::new();
    for (node_id, degree) in in_degree.clone() {
        if degree == 0 {
            stack.push(node_id);
        }
    }

    let mut sorted_order: Vec<String> = Vec::new();
    while let Some(node_id) = stack.pop() {
        sorted_order.push(node_id.clone());
        if let Some(neighbors) = graph.get(&node_id) {
            for neighbor in neighbors {
                let entry = in_degree.entry(neighbor.clone()).or_default();
                *entry -= 1;
                if *entry == 0 {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    if sorted_order.len() == workflow_data.nodes.len() {
        sorted_order
    } else {
        vec![]
    }
}
