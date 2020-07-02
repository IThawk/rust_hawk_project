extern crate serde_yaml;

use crate::utils::file_utils::{read_file, write_file};
use serde::de::DeserializeOwned;
use serde::export::Result::Ok;
use serde::{Deserialize, Serialize};

pub fn read_yml<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
    T: Serialize,
{
    let a = read_file(path.to_string());
    if let Ok(context) = a {
        println!("{}", context);
        let config = serde_yaml::from_str(&context);
        // println!("{:?}", config);
        if let Ok(content) = config {
            return Ok(content);
        }
    }
    Err("err".to_string())
}

pub fn write_yml<T>(path: &str, content: T) -> Result<(), String>
where
    T: Serialize,
{
    // let a = read_file(path.to_string());
    let config_opt = serde_yaml::to_string(&content);
    if let Ok(context) = config_opt {
        println!("{}", context);

        // println!("{:?}", config);
        return write_file(path.to_string(), context, false);
    }
    Err("err".to_string())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn read_test() {
    let point = Point { x: 1.0, y: 2.0 };
    let a = read_file("./config/my.yml".to_string());
    if let Ok(context) = a {
        println!("{}", context);
    }
    let s = serde_yaml::to_string(&point).unwrap();
    assert_eq!(s, "---\nx: 1.0\ny: 2.0");

    let deserialized_point: Point = serde_yaml::from_str(&s).unwrap();
    assert_eq!(point, deserialized_point);
}

