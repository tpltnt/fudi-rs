//! Parse Pure Data Messages using nom.

use crate::PdMessage;
use nom::{is_alphanumeric, IResult};

extern crate rand;
use rand::Rng;

/// Test character for being considered whitespace in FUDI
/// (i.e. ASCII 32 (space), 9 (tab), or 10 (newline)).
fn is_whitespace(c: u8) -> bool {
    (c == 32) || (c == 9) || (c == 10)
}

/// Test character for *not* being considered whitespace in FUDI
/// (i.e. ASCII 32 (space), 9 (tab), or 10 (newline)).
fn is_not_whitespace(c: u8) -> bool {
    !is_whitespace(c)
}

#[cfg(test)]
mod test_supplements {
    use super::*;

    #[test]
    fn test_space() {
        assert!(is_whitespace(b' '));
    }

    #[test]
    fn test_tab() {
        assert!(is_whitespace(b'\t'));
    }

    #[test]
    fn test_newline() {
        assert!(is_whitespace(b'\n'));
    }

    #[test]
    fn test_non_whitespace() {
        // generate random ASCII character
        let mut rng = rand::thread_rng();
        let mut t: u8 = rng.gen_range(0, 128); // ASCII is 7 bit

        // make sure it is not 9,10, or 32
        while (t == 9) || (t == 10) || (t == 32) {
            t = t + 1;
        }

        // check test function
        assert_eq!(is_whitespace(t), false);
    }
}

named!(parse_message<&[u8], (std::vec::Vec<(&[u8], &[u8])>, char)>,
    many_till!(
        pair!(
	    parse_atom,
            take_till!(is_not_whitespace)
        ),
        char!(';')
    )
);

named!(parse_atom<&[u8], &[u8]>,
    take_while!(is_alphanumeric)
);

/// Retrieve Pure Data message from byte payload.
/// *note*: This implementation is incomplete.
fn get_message(payload: &[u8]) -> Result<PdMessage, &str> {
    let res = parse_message(payload);
    if let Ok(parsing_result) = res {
        let (remainder, chunks) = parsing_result;
        let (tokens, semicolon) = chunks;
        if (semicolon != ';') {
            return Err("terminating semicolon is missing");
        }
        // check for potential bang message
        if 1 == tokens.len() {
            let (atom, _) = tokens[0];
            if atom == "bang".as_bytes() {
                return Ok(PdMessage::Bang);
            } else {
                panic!("generic selector-only message not implemented");
            }
        }
        for (i, tmp) in tokens.iter().enumerate() {
            let (atom, _) = tmp; // discard whitespace
        }
    }

    return Err("could not parse payload");
}

#[cfg(test)]
mod test_parser {
    use super::*;

    /// TODO: negative test
    #[test]
    fn parsing_atom() {
        // positive test
        let res = parse_atom(b"bang;\n");
        if let Ok(parsing_result) = res {
            let (remainder, token) = parsing_result;
            let expected: [u8; 2] = [59, 10];
            assert_eq!(remainder, expected);
            let expected = [98, 97, 110, 103];
            assert_eq!(token, expected)
        } else {
            assert!(false);
        }
    }

    /*
        fn parsing_specification_example_messages() {
            // positive test
            let res = parse_message(b"test/blah 123.45314;\n");
            if let Ok(parsing_result) = res {
                let (remainder, token) = parsing_result;
                let expected: [u8; 2] = [59, 10];
                assert_eq!(remainder, expected);
                let expected = [98, 97, 110, 103];
                assert_eq!(token, expected)
            } else {
                assert!(false);
            }
        }
    */
    #[test]
    fn message_from_bang_only_payload() {
        let res = get_message(b"bang;\n");
        match res {
            Ok(message) => println!("{:?}", message),
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn message_from_only_alpha_payload() {
        let res = get_message(b"only alpha;\n");
        match res {
            Ok(message) => println!("{:?}", message),
            Err(msg) => panic!(msg),
        }
    }
}