pub fn remove_placeholder(text_with_placeholder: &String) -> String {
    str::replace(text_with_placeholder, "%placeholder%", "")
}
