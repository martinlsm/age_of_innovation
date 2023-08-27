use crate::{error::create_error, Result};

use std::{collections::HashMap, iter::zip};

use itertools::Itertools;
use serde_json::{json, Map, Number};

pub fn contains_duplicates<T: Eq>(vec: &Vec<T>) -> bool {
    for (i, a) in zip(0.., vec) {
        for b in &vec[i + 1..] {
            if a == b {
                return true;
            }
        }
    }

    false
}

fn json_obj_diff_helper(
    old: &serde_json::Value,
    new: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::Value;

    match (old, new) {
        (Value::Bool(a), Value::Bool(b)) => {
            if a != b {
                Ok(Value::Bool(*b))
            } else {
                Ok(Value::Null)
            }
        }
        (Value::Number(a), Value::Number(b)) => {
            if a != b {
                Ok(Value::Number(b.clone()))
            } else {
                Ok(Value::Null)
            }
        }
        (Value::String(a), Value::String(b)) => {
            if a != b {
                Ok(Value::String(b.clone()))
            } else {
                Ok(Value::Null)
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a != b {
                if a.len() != b.len() {
                    return Err(create_error(
                        "Json arrays must have equal number of elements",
                    ));
                }

                let mut res = Vec::new();
                for (e1, e2) in zip(a, b) {
                    match json_obj_diff_helper(&e1, &e2) {
                        Ok(v) => res.push(v),
                        Err(e) => return Err(e),
                    }
                }

                if res.iter().all(|v| *v == Value::Null) {
                    // Arrays are identical
                    return Ok(Value::Null);
                }

                let res = zip(0.., res)
                    .map(|(i, v)| if v == Value::Null { b[i].clone() } else { v })
                    .collect_vec();

                Ok(Value::Array(res))
            } else {
                Ok(Value::Null)
            }
        }
        (Value::Object(a), Value::Object(b)) => {
            if a != b {
                if a.len() != b.len() {
                    return Err(create_error(
                        "Json objects must have equal number of members",
                    ));
                }

                let mut res = Map::new();
                for (k, v_new) in b {
                    if !a.contains_key(k) {
                        return Err(create_error("Json object must contains same keys"));
                    }
                    let v_old = a.get(k).unwrap();

                    match json_obj_diff_helper(&v_old, &v_new) {
                        Ok(Value::Null) => (),
                        Ok(v) => {
                            res.insert(k.clone(), v);
                        }
                        Err(e) => return Err(e),
                    }
                }

                Ok(Value::Object(res))
            } else {
                Ok(json!({}))
            }
        }
        _ => Err(create_error("Mismatched types")),
    }
}

pub fn json_obj_diff(
    old: &serde_json::Value,
    new: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::Value;

    match (old, new) {
        (Value::Object(_), Value::Object(_)) => (),
        _ => return Err(create_error("Json values are not object types")),
    };

    json_obj_diff_helper(old, new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_include;

    #[test]
    fn contains_duplicates_positive() {
        assert!(contains_duplicates(&vec![1, 2, 3, 2]));
    }

    #[test]
    fn contains_duplicates_negative() {
        assert!(!contains_duplicates(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn contains_duplicates_empty() {
        assert!(!contains_duplicates(&Vec::<i32>::new()));
    }

    #[test]
    fn json_obj_diff_not_valid_for_non_obj_values() {
        let jsons = vec![
            json!([1, 2]),
            json!(2),
            json!("foo"),
            json!(true),
            json!(null),
        ];

        for json1 in &jsons {
            for json2 in &jsons {
                assert!(json_obj_diff(&json1, &json2).is_err());
            }
        }
    }

    #[test]
    fn json_obj_diff_equal_objs_returns_empty_obj() {
        let obj = json!(
            {
                "foo": "old",
                "vec": [1, 2, "baz"]
            }
        );

        let diff = json_obj_diff(&obj, &obj.clone()).unwrap();

        assert_json_include!(actual: diff, expected: json!({}));
    }

    #[test]
    fn json_obj_diff_one_changed_value_is_returned() {
        let obj1 = json!(
            {
                "foo": "old",
                "vec": [1, 2, "baz"]
            }
        );
        let obj2 = json!(
            {
                "foo": "new",
                "vec": [1, 2, "baz"]
            }
        );

        let diff = json_obj_diff(&obj1, &obj2).unwrap();

        let expected = json!({
            "foo": "new"
        });
        assert_json_include!(actual: diff, expected: expected);
    }

    #[test]
    fn json_obj_diff_changed_value_type_returns_error() {
        let obj1 = json!(
            {
                "foo": "bar",
                "key": 2
            }
        );
        let obj2 = json!(
            {
                "foo": "bar",
                "key": true
            }
        );

        assert!(json_obj_diff(&obj1, &obj2).is_err());
    }

    #[test]
    fn json_obj_changed_vec_length_returns_error() {
        let obj1 = json!(
            {
                "foo": "bar",
                "vec": [1, 2, "baz"]
            }
        );
        let obj2 = json!(
            {
                "foo": "bar",
                "vec": [1, 2]
            }
        );

        assert!(json_obj_diff(&obj1, &obj2).is_err());
        assert!(json_obj_diff(&obj2, &obj1).is_err());
    }

    #[test]
    fn json_obj_diff_changed_key_returns_error() {
        let obj1 = json!(
            {
                "foo": "bar"
            }
        );
        let obj2 = json!(
            {
                "baz": "bar"
            }
        );

        assert!(json_obj_diff(&obj1, &obj2).is_err());
    }

    #[test]
    fn json_obj_diff_two_changed_values() {
        let obj1 = json!(
            {
                "foo": "bar",
                "vec": [1, 2, "baz"],
                "key": 1234
            }
        );
        let obj2 = json!(
            {
                "foo": "bar",
                "vec": [1, 2, "changed"],
                "key": 5678
            }
        );

        let diff = json_obj_diff(&obj1, &obj2).unwrap();

        let expected = json!(
            {
                "vec": [1, 2, "changed"],
                "key": 5678
            }
        );

        assert_json_include!(actual: diff, expected: expected);
    }

    #[test]
    fn json_obj_diff_added_or_removed_key_returns_error() {
        let obj1 = json!(
            {
                "foo": "bar",
            }
        );
        let obj2 = json!(
            {
                "foo": "bar",
                "key": 123,
            }
        );

        assert!(json_obj_diff(&obj1, &obj2).is_err());
        assert!(json_obj_diff(&obj2, &obj1).is_err());
    }

    #[test]
    fn json_obj_diff_changed_nested_obj() {
        let obj1 = json!({
            "foo": "bar",
            "nested": {
                "nested_foo": "old",
                "nested_unchanged": 1
            }
        });
        let obj2 = json!({
            "foo": "bar",
            "nested": {
                "nested_foo": "new",
                "nested_unchanged": 1
            }
        });

        let diff = json_obj_diff(&obj1, &obj2).unwrap();

        let expected = json!({
            "nested": {
                "nested_foo": "new"
            }
        });
        assert_json_include!(actual: diff, expected: expected);
    }

    #[test]
    fn json_obj_diff_nested_objs_do_not_match() {
        let obj1 = json!({
            "nested": {
                "nested_foo": "bar",
                "nested_baz": 1
            }
        });
        let obj2 = json!({
            "nested": {
                "nested_foo": "bar",
                "nested_baz": "changed type", // Illegal
            }
        });

        assert!(json_obj_diff(&obj1, &obj2).is_err());
    }

    #[test]
    fn json_obj_diff_nested_obj_in_vec_is_changed() {
        let obj1 = json!(
            {
                "foo": 1,
                "changed_vec": [1, 2, { "nested_foo": 5, "nested_bar": "unchanged" }],

            }
        );
        let obj2 = json!(
            {
                "foo": 1,
                "changed_vec": [1, 2, { "nested_foo": 6, "nested_bar": "unchanged" }]
            }
        );

        let diff = json_obj_diff(&obj1, &obj2).unwrap();

        let expected = json!(
            {
                "changed_vec": [1, 2, { "nested_foo": 6 }]
            }
        );
        assert_json_include!(actual: diff, expected: expected);
    }

    #[test]
    fn json_obj_diff_vec_containing_obj_is_changed_via_another_elem() {
        let obj1 = json!({
            "vec": [1, { "nested_foo": "bar" } ]
        });
        let obj2 = json!({
            "vec": [2, { "nested_foo": "bar" } ]
        });

        let diff = json_obj_diff(&obj1, &obj2).unwrap();

        let expected = json!(
            {
                "vec": [2, {}]  // Second elem is empty bacause it is unchanged
            }
        );
        assert_json_include!(actual: diff, expected: expected);
    }
}
