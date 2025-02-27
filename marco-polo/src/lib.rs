// This function takes a string slice as input and returns a string as output.
// If the input is "Marco", it returns "Polo". Otherwise, it returns "What's
// your name?"
pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}
