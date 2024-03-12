pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!"
    }

    let contains_letters = message.chars().any(char::is_alphabetic);
    let shorting = contains_letters && message.to_uppercase() == message;
    let question = message.trim().ends_with("?");

    match (shorting, question) {
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever."
        
    }

}
