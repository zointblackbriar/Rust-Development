//This function returns two outcomes from Result
fn error_check(check: bool) -> Result <i8, &'static str>{
    if check == true {
        return Err("this is an error"); 
    } else {
        return Ok(1)
    }
} 

//this function takes a result and handles it with a match statement
fn describe_result(result: Result<i8, &'static str>) {
    match result {
        Ok(x) => println!("It's a result of: {}", x), 
        Err(x) => println!("{}", x)
    }
} 

fn main() {
    println!("hello main rust function");
}