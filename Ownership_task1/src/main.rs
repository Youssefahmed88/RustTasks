fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::from(str1);
    result.push_str(str2);
    result
}

fn main(){
    let first = String::from("Hello,");
    let sec = String::from("Youssef");
    let concat = concatenate_strings(&first, &sec);
    println!("Concat String: {}", concat);
}

// In concatenate_strings():
// - We receive string slices (&str) as references to avoid moving ownership
// - We create a new String by copying the data that str1 (a reference) points to, which is stored on the heap
// - Then we append str2 to the new String
// - We return the new String transferring ownership to the caller

// In main():
// - We create String variables stored on the heap.
// - We pass references (&first, &sec) to avoid moving ownership.
// - We store the returned owned String in 'concat'.
// - Finally, we print the concatenated String.
