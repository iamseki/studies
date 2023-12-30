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

fn return_a_string_literal() -> &'static str {
    "Hello world"
}

fn return_a_string_mut(output: &mut String) {
    output.replace_range(.., "Hello world"); // replace the entire string in output with "Hello world"
}

fn main() {
    return_a_string();
    
    println!("
return_a_string() -> &String is unsafe to run in the compile eyes because there's no way to ensure that the returned reference will live long enough.
One way to do so in a safety way is to actually returns String instead of &String
Moving the ownership of the String out of the function.
");
    
    return_a_string_literal();
    println!("
return_a_string_literal() -> &'static str also works. The value returned from function using the 'static creates a variable that lives forever
in the stack.    
");

    let mut word = String::new();
    return_a_string_mut(&mut word);
    println!("
The function return_a_string_mut(output: &mut String) using a mutation approach.
In rust we need to explicitly call the function as said as it's parameter is mutable: return_a_string_mut(&mut word);
")

}

