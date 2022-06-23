mod macros;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Self {
        Json::String(s.to_string())
    }
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

fn main() {
    let width = 4.0;
    let desc =
        json!({
            "width": width,
            "height": (width * 9.0 / 4.0)
        });

    println!("{desc:?}")
}


#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}


#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!(
        [
        {
            "pitch": 440.0
        }
            ]
    );

    let hand_coded_value = Json::Array(vec![
                    Json::Object(Box::new(vec![
                        ("pitch".to_string(), Json::Number(440.0))
                    ].into_iter().collect()))
    ]);

    assert_eq!(macro_generated_value, hand_coded_value);
}