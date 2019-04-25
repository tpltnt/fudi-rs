//! Parse Pure Data Messages using nom.

use crate::{GenericMessage, PdMessage};
use nom::{alphanumeric, digit, float};

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

/// Test for valid character in atom (i.e. not whitespace or semicolon).
fn valid_atom_character(c: u8) -> bool {
    is_not_whitespace(c) || c != 59
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

    #[test]
    fn valid_atom_chars() {
        assert!(valid_atom_character(b';'));
    }
}

named!(parse_message<&[u8], (std::vec::Vec<(((std::option::Option<f32>, std::option::Option<&[u8]>), std::option::Option<&[u8]>), &[u8])>, char)>,
    many_till!(
        pair!(
	    parse_atom,
            take_till!(is_not_whitespace)
        ),
        char!(';')
    )
);

// An atom is either an integer, a float, or a string
named!(parse_atom<&[u8], ((std::option::Option<f32>, std::option::Option<&[u8]>), std::option::Option<&[u8]>)>,
    pair!(
        pair!(
            opt!(float),
            opt!(digit)
	),
        opt!(alphanumeric)
    )
);

/// Retrieve Pure Data message from byte payload.
/// *note*: This implementation is incomplete.
pub fn get_message(payload: &[u8]) -> Result<PdMessage, &str> {
    let res = parse_message(payload);
    if let Ok(parsing_result) = res {
        let (remainder, chunks) = parsing_result;
        let (tokens, semicolon) = chunks;
        if semicolon != ';' {
            return Err("terminating semicolon is missing");
        }

        // check for potential bang or float message
        if 1 == tokens.len() {
            // extract relevant data (types)
            let (msg_parts, _) = tokens[0]; // separate potental atoms from whitespace
            let (number, word) = msg_parts; // split into potential numbers and strings

            // text -> potential bang message
            if let Some(atom) = word {
                if atom == "bang".as_bytes() {
                    return Ok(PdMessage::Bang);
                }
                // generic message with only selector
                return Ok(PdMessage::Generic(GenericMessage {
                    selector: String::from_utf8(atom.to_vec()).unwrap(),
                    atoms: vec![],
                }));
            }
            // number -> float message
            let (f, digits) = number; // separate float from integer
            if let Some(atom) = f {
                return Ok(PdMessage::Float(atom));
            }
            if let Some(atom) = digits {
                // digits need to be converted to integer
                if 45 == atom[0] {
                    // negative sign/prefix
                    let word = String::from_utf8(atom[1..].to_vec()).unwrap();
                    let as_int = word.parse::<u32>();
                    if !as_int.is_err() {
                        let val = as_int.unwrap() as f32;
                        return Ok(PdMessage::Float(val * -1.0));
                    }
                } else {
                    let word = String::from_utf8(atom.to_vec()).unwrap();
                    let as_int = word.parse::<u32>();
                    if !as_int.is_err() {
                        let val = as_int.unwrap() as f32;
                        return Ok(PdMessage::Float(val * -1.0));
                    }
                }
            }
        }

        // check for symbol and float messages
        if 2 == tokens.len() {
            // extract relevant data (types)
            let (msg_parts, _) = tokens[0]; // separate potental selector from whitespace
            let (_, word) = msg_parts; // split into potential numbers and strings

            // text -> selector
            if let Some(atom) = word {
                // handle float message
                if atom == "float".as_bytes() {
                    let (msg_parts, _) = tokens[1];
                    let (number, _) = msg_parts;
                    // number -> float message
                    let (f, digits) = number; // separate float from integer
                    if let Some(atom) = f {
                        return Ok(PdMessage::Float(atom));
                    }
                    if let Some(atom) = digits {
                        // digits need to be converted to integer
                        if 45 == atom[0] {
                            // negative sign/prefix
                            let word = String::from_utf8(atom[1..].to_vec()).unwrap();
                            let as_int = word.parse::<u32>();
                            if !as_int.is_err() {
                                let val = as_int.unwrap() as f32;
                                return Ok(PdMessage::Float(val * -1.0));
                            }
                        } else {
                            let word = String::from_utf8(atom.to_vec()).unwrap();
                            let as_int = word.parse::<u32>();
                            if !as_int.is_err() {
                                let val = as_int.unwrap() as f32;
                                return Ok(PdMessage::Float(val * -1.0));
                            }
                        }
                    }
                }

                // handle symbol message
                if atom == "symbol".as_bytes() {
                    let (msg_parts, _) = tokens[1];
                    let (_, word) = msg_parts;
                    if let Some(atom) = word {
                        return Ok(PdMessage::Symbol(String::from_utf8(atom.to_vec()).unwrap()));
                    }

                    panic!("parsing symbol message not yet implemented");
                }
            }
        }

        // message with multiple atoms
        let mut atoms: Vec<String> = vec![];
        for tmp in tokens.iter() {
            let (msg_parts, _) = tmp; // discard whitespace
            let (_, word) = msg_parts;
            // handle only text atoms
            if let Some(atom) = word {
                atoms.push(String::from_utf8(atom.to_vec()).unwrap());
            }
        }

        // valid message, but no pre-defined type
        return Ok(PdMessage::Generic(GenericMessage {
            selector: atoms[0].clone(),
            atoms: atoms[1..].to_vec(),
        }));
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
            let (remainder, tokens) = parsing_result;
            let expected: [u8; 2] = [59, 10];
            assert_eq!(remainder, expected);
            // unpack all options
            let (_, text) = tokens;
            if let Some(sym) = text {
                let expected = [98, 97, 110, 103];
                assert_eq!(sym, expected);
            } else {
                assert!(false);
            }
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
            Ok(message) => assert_eq!("bang;\n", message.to_text()),
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn message_from_only_alpha_payload() {
        let res = get_message(b"selector;\n");
        match res {
            Ok(message) => assert_eq!("selector;\n", message.to_text()),
            Err(msg) => panic!(msg),
        }

        let res = get_message(b"only alpha msg;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Generic(_) => assert_eq!("only alpha msg;\n", message.to_text()),
                _ => panic!("unexpected message type"),
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn message_from_float_payload() {
        // float messages can be implied
        let res = get_message(b"39;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Float(_) => assert_eq!("float 39;\n", message.to_text()),
                _ => panic!("float message expected, different type detected"),
            },
            Err(msg) => panic!(msg),
        }

        let res = get_message(b"-3;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Float(_) => assert_eq!("float -3;\n", message.to_text()),
                _ => panic!("float message expected, different type detected"),
            },
            Err(msg) => panic!(msg),
        }

        let res = get_message(b"float 3;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Float(_) => assert_eq!("float 3;\n", message.to_text()),
                _ => panic!("unexpected message type"),
            },
            Err(msg) => panic!(msg),
        }

        let res = get_message(b"float -5.7;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Float(_) => assert_eq!("float -5.7;\n", message.to_text()),
                _ => panic!("unexpected message type"),
            },
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn message_from_symbol_payload() {
        let res = get_message(b"symbol foo;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Symbol(_) => assert_eq!("symbol foo;\n", message.to_text()),
                _ => panic!("symbol message expected, different type detected"),
            },
            Err(msg) => panic!(msg),
        }

        let res = get_message(b"la la;\n");
        match res {
            Ok(message) => match message {
                PdMessage::Symbol(_) => {
                    panic!("non-symbol message expected, symbol message detected")
                }
                _ => (),
            },
            Err(msg) => panic!(msg),
        }
    }

}
