use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
pub enum JsonResult {
    Map(HashMap<String, JsonResult>),
    Array(Vec<JsonResult>),
    String(String),
    Boolean(bool),
    Number(i32),
    None,
}

fn main() {
    // now lets write one simple json string that contains one object
    let dummy_json: String =
        String::from("{\"qux\": { \"quux\": [20, \"bhaskar\"]}, \"name\" : \"bhaskar\"}");
    let parsed_json: JsonResult = parse_any(dummy_json);
    print!("{:?}", { parsed_json });
}

fn parse_any(json: String) -> JsonResult {
    let mut iterator = json.chars().peekable();
    // print!("{:?}", {iterator});
    if iterator.peek().is_none() {
        return JsonResult::None;
    }
    return parse_object(&mut iterator);
}

fn parse_object(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut map: HashMap<String, JsonResult> = HashMap::new();
    if it.peek().is_none() {
        return JsonResult::None;
    }
    consume(it);
    loop {
        while !it.peek().is_none() && *it.peek().unwrap() == ' ' {
            it.next();
        }
        match it.peek() {
            Some(ch) => {
                if *ch == '}' {
                    consume(it);
                    return JsonResult::Map(map);
                }
                let key = parse_value(it);
                parse_colon(it);
                let value = parse_value(it);
                parse_comma(it);
                match key {
                    JsonResult::String(key) => {
                        map.insert(key, value);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    return JsonResult::Map(map);
    // print!("{:?}", map);
}

fn parse_value(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    if it.peek().is_none() {
        return JsonResult::None;
    }
    while !it.peek().is_none() && *it.peek().unwrap() == ' ' {
        it.next();
    }
    match it.peek().unwrap() {
        '[' => parse_array(it),
        '{' => {
            // print!("parsing object....");
            parse_object(it)
            // return JsonResult::None;
        }
        '"' => {
            // print!("parsing string....");
            return parse_string(it);
        }
        't' => {
            // print!("parsing true......");
            return parse_true(it);
        }
        'f' => {
            // print!("parsing false.....");
            return parse_false(it);
        }
        _ => {
            // print!("parsing numbers...");
            return parse_number(it);
        }
    }
}
fn consume(it: &mut Peekable<impl Iterator<Item = char>>) {
    it.next();
}
fn parse_colon(it: &mut Peekable<impl Iterator<Item = char>>) {
    while !it.peek().is_none() && *it.peek().unwrap() != ':' {
        it.next();
    }
    consume(it);
}
fn parse_comma(it: &mut Peekable<impl Iterator<Item = char>>) {
    if *it.peek().unwrap() == '}' {
        return;
    }
    while !it.peek().is_none() && *it.peek().unwrap() != ',' {
        it.next();
    }
    consume(it);
}
fn parse_number(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut result: String = String::new();
    while !it.peek().is_none()
        && *it.peek().unwrap() != ','
        && *it.peek().unwrap() != ']'
        && *it.peek().unwrap() != '}'
    {
        result.push(it.next().unwrap());
    }
    // consume(it);
    let number_result: i32 = result.trim().parse().unwrap_or(-1);
    return JsonResult::Number(number_result);
}
fn parse_string(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut result: String = String::new();
    consume(it);
    while *it.peek().unwrap() != '"' {
        result.push(it.next().unwrap());
    }
    consume(it);
    return JsonResult::String(result);
}
fn parse_true(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut count: i32 = 0;
    while !it.peek().is_none() && count != 4 {
        it.next();
        count += 1;
    }
    return JsonResult::Boolean(true);
}
fn parse_false(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut count: i32 = 0;
    while !it.peek().is_none() && count != 5 {
        it.next();
        count += 1;
    }
    return JsonResult::Boolean(false);
}
fn parse_array(it: &mut Peekable<impl Iterator<Item = char>>) -> JsonResult {
    let mut items: Vec<JsonResult> = Vec::new();
    consume(it);
    while !it.peek().is_none() {
        match *it.peek().unwrap() {
            ',' => {
                consume(it);
            }
            ' ' => {
                consume(it);
            }
            '[' => {
                items.push(parse_array(it));
            }
            ']' => {
                consume(it);
                break;
            }
            _ => {
                items.push(parse_value(it));
            }
        }
    }
    return JsonResult::Array(items);
}
