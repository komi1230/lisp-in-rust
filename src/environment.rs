use std::collections::VecDeque;

fn include_float(tokens: &mut VecDeque<String>) -> bool {
    for i in tokens {
        if let Ok(tmp) = i.parse::<i32>() {
            return false;
        }
    }

    true
}

fn include_string(tokens: &mut VecDeque<String>) -> bool {
    for i in tokens {
        if let Err(tmp) = i.parse::<f64>() {
            return true;
        }
    }

    false
}

// Basically calculate as Integer
// But when there is any float in tokens, calculate as float
pub fn add(args: &mut VecDeque<String>) -> String {
    if include_string(args) {
        panic!("List include not number");
    }

    // calculate as float
    if include_float(args) {
        let result_float: f64 = args
            .iter()
            .map(|k| k.parse::<f64>().unwrap())
            .fold(0.0, |s, i| s + i);
        return result_float.to_string();
    }

    // otherwise
    let result_int: i32 = args
        .iter()
        .map(|k| k.parse::<i32>().unwrap())
        .fold(0, |sum, i| sum + i);
    result_int.to_string()
}

pub fn sub(args: &mut VecDeque<String>) -> String {
    if include_string(args) {
        panic!("List include not number");
    }

    // calculate as float
    if include_float(args) {
        let head_float: f64 = args.pop_front().unwrap().parse::<f64>().unwrap();
        if args.len() == 0 {
            return head_float.to_string();
        }

        let result_float: f64 = args
            .iter()
            .map(|k| k.parse::<f64>().unwrap())
            .fold(head_float, |s, i| s + i);
        return result_float.to_string();
    }

    let head_int: i32 = args.pop_front().unwrap().parse::<i32>().unwrap();
    if args.len() == 0 {
        return head_int.to_string();
    }

    let result_int: i32 = args
        .iter()
        .map(|k| k.parse::<i32>().unwrap())
        .fold(head_int, |s, i| s + i);
    result_int.to_string()
}

pub fn mul(args: &mut VecDeque<String>) -> String {
    if include_string(args) {
        panic!("List include not number");
    }

    // calculate as float
    if include_float(args) {
        let result_float: f64 = args
            .iter()
            .map(|k| k.parse::<f64>().unwrap())
            .fold(0.0, |s, i| s * i);
        return result_float.to_string();
    }

    // otherwise
    let result_int: i32 = args
        .iter()
        .map(|k| k.parse::<i32>().unwrap())
        .fold(0, |s, i| s * i);
    result_int.to_string()
}

pub fn div(args: &mut VecDeque<String>) -> String {
    if include_string(args) {
        panic!("List include not number");
    }

    // calculate as float
    if include_float(args) {
        let head_float: f64 = args.pop_front().unwrap().parse::<f64>().unwrap();
        if args.len() == 0 {
            return head_float.to_string();
        }

        let result_float: f64 = args
            .iter()
            .map(|k| k.parse::<f64>().unwrap())
            .fold(head_float, |s, i| s / i);
        return result_float.to_string();
    }

    let head_int: i32 = args.pop_front().unwrap().parse::<i32>().unwrap();
    if args.len() == 0 {
        return head_int.to_string();
    }

    let result_int: i32 = args
        .iter()
        .map(|k| k.parse::<i32>().unwrap())
        .fold(head_int, |s, i| s / i);
    result_int.to_string()
}
