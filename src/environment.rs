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
            .fold(0.0, |sum, i| sum + i);
        return result_float.to_string();
    }

    // otherwise
    let result_int: i32 = args
        .iter()
        .map(|k| k.parse::<i32>().unwrap())
        .fold(0, |sum, i| sum + i);
    return result_int.to_string();
}
