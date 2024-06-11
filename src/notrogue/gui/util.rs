pub fn wrap_text(string: &str, len: u32) -> Vec<String> {
    let mut ret = Vec::new();
    let mut current_string = String::new();
    for character in string.chars() {
        current_string.push(character);
        if current_string.len() == len as usize {
            ret.push(current_string.clone());
            current_string = String::from(" ");
        }
    }
    if current_string != String::from(" ") {
        ret.push(current_string);
    }
    ret
}