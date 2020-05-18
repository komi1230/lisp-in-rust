use std::collections::VecDeque;


fn add_float(tokens: &mut VecDeque<String>) -> i32 {
    let token = tokens.pop_front().unwrap();
    
    if tokens.len() == 0 {
        return 0;
    }
    
    if let Ok(value_float) = token.parse::<f64>() {
        return value_float + add_int(tokens)
    } else {
        panic!("Not Integer")
    }
}


// Basically calculate as Integer
// But when there is any float in tokens, calculate as float
pub fn add(tokens: &mut VecDeque<String>) -> String {
    let mut result_int = 0;
    let mut result_float = 0.0;
    let mut flag_int = true;
    while tokens.len() > 0 {
        let token = tokens.pop_front().unwrap();

        
        if let Ok(value_int) = token.parse::<i32>() {
            result_int += value_int;
        }
        
        if let Ok(value_float) = token.parse::<f64>() {
            result_float += value_float;
        }
    }
    if result_float == 0.0 {
        return result_int.to_string();
    }

    return 
}
