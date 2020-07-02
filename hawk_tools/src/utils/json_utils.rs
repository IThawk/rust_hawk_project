extern crate serde_json;

use crate::utils::file_utils::{read_file, write_file};
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
use serde::export::Result::Ok;
use serde::{Deserialize, Serialize};

///json object to string
/// ```
/// use hawk_tools::utils::json_utils::json_to_string;
/// let point = Point { x: 1.0, y: 2.0 };
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// struct Point {
///     x: f64,
///     y: f64,
/// }
/// let s = json_to_string::<Point>(&point);
/// ```
///
pub fn json_to_string<T>(object: &T) -> Result<String, String>
where
    T: DeserializeOwned,
    T: Serialize,
{
    let a = serde_json::to_string(object);
    if let Ok(context) = a {
        println!("context : {}", context);
        return Ok(context);
    }
    Err("err".to_string())
}

///string to object
/// ```
/// use hawk_tools::utils::json_utils::string_to_object;
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// struct Point {
///     x: f64,
///     y: f64,
/// }
/// let point = "{\"x\":1.0,\"y\":2.0}";
/// let s = string_to_object::<Point>(&point);
/// ```
///
pub fn string_to_object<T>(obj_str: &str) -> Result<T, String>
where
    T: DeserializeOwned,
    T: Serialize,
    T: Debug,
{
    let config_opt = serde_json::from_str::<T>(obj_str);
    if let Ok(context) = config_opt {
        println!("{:?}", context);
        return Ok(context);
    }
    Err("err".to_string())
}

/// PartialEq this trait to compare object
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn to_json_test() {
    let point = Point { x: 1.0, y: 2.0 };
    let s = json_to_string::<Point>(&point);
    println!("{:?}", s);
    assert_eq!(s.unwrap(), "{\"x\":1.0,\"y\":2.0}");
}

#[test]
fn string_to_object_test() {
    let point = "{\"x\":1.0,\"y\":2.0}";
    let a = string_to_object::<Point>(point);
    println!("{:?}", a);
    assert_eq!(a.unwrap(), Point { x: 1.0, y: 2.0 });
}
