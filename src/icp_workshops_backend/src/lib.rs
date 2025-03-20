#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn calculate(a:i32, b:i32, operator: String) -> String{

    let res = match operator.as_str(){
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => if b != 0 {Some(a / b)} else {None},
        _ => None
    };

    match res{
        Some(value) => format!("Results: {}", value),
        None => "Invalid operator or divison by zero!".to_string(),
    }
}
