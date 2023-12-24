// this function will result in compile errors
// Rust can't make sure that the reference returned &String lives long enough
/*
fn return_a_string() -> &String {
    let s = String::from("Hello world!");
    &s
}
*/

fn return_a_string() -> String {
    let s = String::from("Hello world!");
    s
}

fn main() {
    return_a_string();
    print!("
return_a_string() -> &String is unsafe to run in the compile eyes because there's no way to ensure that the returned reference will live long enough.
One way to do so in a safety way is to actually returns String instead of &String
Moving the ownership of the String out of the function.
")
  
}

