/// An incomplete implementation of Pure Data message types.
/// TODO: implement list, pointer and custom message types
enum PdMessage {
    Float(f32),
    Symbol(String),
    Bang,
}

impl PdMessage {
    /// Generate a message string for the (given) message type
    fn to_text(&self) -> String {
        let mut payload: String;
        match &self {
	    PdMessage::Float(f) => payload = format!("float {}", f),
	    PdMessage::Symbol(word) => payload = format!("symbol {}", word),
	    PdMessage::Bang => payload = String::from("bang"),
	}
	payload = format!("{};", payload);
	return payload;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_float_message() {
        let msg = PdMessage::Float(2.974);
	assert_eq!(String::from("float 2.974;"), msg.to_text());
    }

    #[test]
    fn generate_symbol_message() {
        let msg = PdMessage::Symbol(String::from("foobar"));
	assert_eq!(String::from("symbol foobar;"), msg.to_text());
    }

    #[test]
    fn generate_bang_message() {
        let msg = PdMessage::Bang;
	assert_eq!(String::from("bang;"), msg.to_text());
    }
}