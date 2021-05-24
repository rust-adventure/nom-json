use std::{fs, io};

use nom_json::*;

#[test]
fn parses_value_object() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
    let (input, value) = json("{\"akey\":\"avalue\"}")?;
    assert_eq!(input, "");
    assert_eq!(
        value,
        Value::Object {
            members: vec![Member {
                key: "akey",
                value: Value::String("avalue")
            }]
        }
    );
    Ok(())
}
#[test]
fn parses_value_array() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
    let (input, value) = json("[\"one\",\"two\"]")?;
    assert_eq!(input, "");
    assert_eq!(
        value,
        Value::Array(vec![Value::String("one"), Value::String("two")])
    );
    Ok(())
}
#[test]
fn parses_value_string() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
    let (input, value) = json("\"astring\"")?;
    assert_eq!(input, "");
    assert_eq!(value, Value::String("astring"));
    Ok(())
}
#[test]
fn parses_value_number() -> Result<(), nom::Err<nom::error::Error<&'static str>>> {
    let (input, value) = json("2")?;
    assert_eq!(input, "");
    assert_eq!(value, Value::Number(2.0));
    Ok(())
}

#[test]
fn json_file_tests() {
    let dirs = fs::read_dir("tests/test_parsing").expect("Failed to read directory");
    for maybe_entry in dirs {
        let entry = maybe_entry.expect("to exist");

        let file_name = entry.file_name();

        let ch = &file_name
            .into_string()
            .expect("valid filename")
            .chars()
            .next()
            .unwrap();

        match ch {
            'y' => {
                let file = fs::read_to_string(entry.path()).expect("file to exist");

                let val = json(&file);
                match val {
                    Ok((input, v)) => {
                        // dbg!(v);
                    }
                    Err(e) => {
                        dbg!(e);
                        panic!("failed at {:?}", entry.file_name());
                    }
                }
            }
            'n' => {
                let file = fs::read_to_string(entry.path());
                match &file {
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::InvalidData => {
                                // ignoring invalid utf-8 data
                            }
                            err => panic!(err),
                        }
                    }
                    _ => {
                        let f = file.expect("msg");
                        let val = json(&f);
                        match val {
                            Ok((input, v)) => {
                                dbg!(v);
                                panic!(
                                    "succeeded when it should have failed at {:?}",
                                    entry.file_name()
                                );
                            }
                            Err(_) => {}
                        }
                    }
                };
            }
            'i' => {}
            _ => {}
        }
        // dbg!(file_name);
        // assert_eq!(true, false);
    }
}
