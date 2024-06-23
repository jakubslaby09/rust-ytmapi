use std::cmp::max;

use serde_json::Value;

use crate::{requests::{create_api_request, endpoint_context}, Client};

#[tokio::main]
#[cfg(feature="findpointers")]
#[test]
async fn main() {
    let client = Client::init().await.unwrap();

    // album(&client, "MPREb_2VNCkyjHbew", Value::String("Odyssey".to_string())).await;
    // album(&client, "MPREb_LJSVi8szMQL", Value::String("Zeit".to_string())).await;

    let res = browse("ALBUM", "MPREb_LJSVi8szMQL");
    dbg!(find_common_pointer(&res, &[Value::String("Zeit".to_string()), Value::String("Giftig".to_string())], 32));
}

async fn browse(type_name: &str, id: &str) -> Value {
    create_api_request(
        &client.config, "browse", endpoint_context(type_name, id)
    ).await.unwrap()
}

fn flatten_json<F: FnMut(String, &Value)>(json: &Value, depth: usize, start_pointer: Option<String>, callback: &mut F) {
    let start_pointer = start_pointer.unwrap_or("".to_string());
    match json {
        Value::Object(map) if depth != 0 => for (name, value) in map {
            flatten_json(value, depth - 1, Some(format!("{start_pointer}/{name}")), callback)
        },
        Value::Array(vec) if depth != 0 => for (index, value) in vec.iter().enumerate() {
            flatten_json(value, depth - 1, Some(format!("{start_pointer}/{index}")), callback)
        },
        other => callback(start_pointer, other),
    }
}

fn find_pointers(json: &Value, value: &Value, max_depth: usize) -> Vec<String> {
    let mut res = vec![];
    flatten_json(json, max_depth, None, &mut |pointer, v| {
        if v == value {
            res.push(pointer)
        }
    });
    res
}

fn find_common_poisnter(json: &Value, values: &[Value], max_depth: usize) -> Option<String> {
    let mut pointers: Vec<Vec<String>> = values.iter().map(|value| {
        find_pointers(json, value, max_depth)
    }).collect();

    // let max_length: usize = pointers.iter().flatten().fold(0, 
    //     |prev_max, pointer| max(prev_max, pointer.len())
    // );
    let mut min_score: usize = 0;
    let mut res = vec![];
    for (value_index, value_pointers) in pointers.clone().into_iter().enumerate() {
        let (common_count, pointer) = value_pointers.into_iter().fold((0, "".to_string()), |(last_common_count, last_pointer), pointer| {
            let common_count = pointers.iter().enumerate().filter(|(i, other_value_pointers)| {
                if *i == value_index {
                    return false;
                }
                other_value_pointers.iter().any(|p| {
                    let similarity = common_pointer_similarity(&pointer, &p);
                    if similarity > min_score {
                        min_score = similarity;
                    }
                    min_score == similarity
                })
            }).count();
            if common_count >= last_common_count {
                (common_count, pointer)
            } else {
                (last_common_count, last_pointer)
            }
        });
        if common_count == 0 {
            return None;
        }
        res.push(pointer);
    }
    res.first().map(|pointer| pointer[0..min_score].to_string())
}

fn common_pointer_similarity(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(a, b)| a == b).count()
}