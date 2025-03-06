#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[ic_cdk::query]
fn calculate(a: i32, b: i32, operator: String) -> String {
    let result = match operator.as_str() {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => if b != 0 { Some (a / b) } else { None },
        "%" => if b != 0 { Some (a % b) } else { None },
        _ => None, 
    };
    
    match result {
        Some(value) => format!("Result: {}", value),
        None => "Wrong operator or division by zero".to_string(),
    }
}
