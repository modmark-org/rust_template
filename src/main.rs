use std::env;
use std::io::{self, Read};

use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let action = &args[0];
    match action.as_str() {
        "manifest" => manifest(),
        "transform" => transform(&args[1], &args[2]),
        other => {
            eprintln!("Invalid action {other}")
        }
    }
}

fn manifest() {
    print!(
        "{}",
        serde_json::to_string(&json!(
            {
            "name": "template",
            "version": "0.1",
            "description": "This is a template package.",
            "transforms": [
                {
                    "from": "template",
                    "to": ["html"],
                    "arguments": [],
                }
            ]
            }
        ))
        .unwrap()
    );
}

fn transform(from: &str, to: &str) {
    match from {
        "template" => transform_template(to),
        other => {
            eprintln!("Package does not support {other}");
        }
    }
}

fn transform_template(to: &str) {
    let input: Value = {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        serde_json::from_str(&buffer).unwrap()
    };

    match to {
        "html" => {
            let content = input["data"].as_str().unwrap();

            let output = json!([
                {"name": "raw", "data": "<a>"},
                {"name": "raw", "data": content},
                {"name": "raw", "data":  "</a>"},
            ]);

            print!("{output}");
        }
        other => {
            eprintln!("Cannot convert template to {other}");
        }
    }
}
