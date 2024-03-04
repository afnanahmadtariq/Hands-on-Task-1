fn main(){
    let string1: String = String::from("Hello, ");
    let string2: String = String::from("World");
    let concatenated_string: String = concatenate_strings(&string1[0..7], &string2[0..5]);
    println!( "Concatenated String: {}", concatenated_string );
}

fn concatenate_strings(string1: &str, string2: &str) -> String{
    let mut result: String = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}