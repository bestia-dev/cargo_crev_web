use std::error::Error;
use std::fmt::{self, Display};
use std::io;
use std::io::Write;
use std::str;
use std::string::FromUtf8Error;

// from https://github.com/kornelski/rust_urlencoding/blob/optimize/src/lib.rs

/// UrlEncodedString
/// url_encoding is NOT done on the entire url !
/// It must be done on every part of the url path separately.
/// url examples: scheme://prefix.domain:port/path1/path2/path3/filename
/// for example <website/crate/čćžšđ>
/// if I url_encode everything the / becomes %2F. We don't want that
/// This looks like work for a macro, but I don't know yet about macros.
/// I will do a
/// it is just a normal string, but because the type is special
/// I can guarantee that no normal string can go into a function that needs
/// TODO: maybe I will use references one they when I understand better the lifetimes
/// or Cow, to have both possibilities: String and &str. If is needed there is a lazy
/// allocation. Or not if not needed.
pub struct UrlEncodedString {
    /// this field is private.
    /// it will be accessed with a method
    enc: String,
    has_filename: bool,
}
impl UrlEncodedString {
    #[allow(unused)]
    /// the only place to encode Url
    /// scheme://prefix.domain:port/path1/path2/path3/filename
    /// TODO: maybe lazy encoding, only when needed?
    pub fn new_with_domain(
        scheme: Option<&str>,
        prefix: Option<&str>,
        domain: Option<&str>,
        port: Option<&str>,
        path1: Option<&str>,
        path2: Option<&str>,
        path3: Option<&str>,
        filename: Option<&str>,
    ) -> Self {
        let mut enc = String::with_capacity(20);
        let mut has_filename = false;
        if let Some(scheme) = scheme {
            enc.push_str(scheme);
            enc.push_str("://");
        }
        if let Some(prefix) = prefix {
            enc.push_str(prefix);
            enc.push_str(".");
        }
        // domains always have trailing slashes
        if let Some(domain) = domain {
            enc.push_str(domain);
            if let Some(port) = port {
                enc.push(':');
                enc.push_str(port);
            }
            enc.push('/');
        }
        Self::push_paths_and_filename(&mut enc, &mut has_filename, path1, path2, path3, filename);
        //return
        Self { enc, has_filename }
    }

    /// private fn to push paths and filename
    fn push_paths_and_filename(
        enc: &mut String,
        has_filename: &mut bool,
        path1: Option<&str>,
        path2: Option<&str>,
        path3: Option<&str>,
        filename: Option<&str>,
    ) {
        // trailing slashes are recommended
        if let Some(path1) = path1 {
            enc.push_str(&url_encode(path1));
            enc.push('/');
        }
        if let Some(path2) = path2 {
            enc.push_str(&url_encode(path2));
            enc.push('/');
        }
        if let Some(path3) = path3 {
            enc.push_str(&url_encode(path3));
            enc.push('/');
        }
        // filename is without trailing slash. can be None
        if let Some(filename) = filename {
            *has_filename = true;
            enc.push_str(&url_encode(filename));
        }
    }
    /// absolute local route, starts with /
    pub fn new_abs_local_route(
        path1: Option<&str>,
        path2: Option<&str>,
        path3: Option<&str>,
        filename: Option<&str>,
    ) -> Self {
        let mut enc = String::with_capacity(20);
        let mut has_filename = false;
        enc.push('/');
        Self::push_paths_and_filename(&mut enc, &mut has_filename, path1, path2, path3, filename);
        //return
        Self { enc, has_filename }
    }
    #[allow(unused)]
    /// relative local route, does NOT start with /
    pub fn new_rel_local_route(
        path1: Option<&str>,
        path2: Option<&str>,
        path3: Option<&str>,
        filename: Option<&str>,
    ) -> Self {
        let mut enc = String::with_capacity(20);
        let mut has_filename = false;
        Self::push_paths_and_filename(&mut enc, &mut has_filename, path1, path2, path3, filename);
        //return
        Self { enc, has_filename }
    }
    #[allow(unused)]
    /// path has always training slashes
    /// if the filename is already pushed, this
    pub fn push_path_part(&mut self, path_part: &str) -> Result<(), &'static str> {
        if self.has_filename == true {
            return Err("The url has already the end with filename.");
        } else {
            self.enc.push_str(&url_encode(path_part));
            self.enc.push('/');
            //return
            Ok(())
        }
    }
    #[allow(unused)]
    /// filename has never trailing slashes
    pub fn push_filename(&mut self, file_name: &str) -> Result<(), &'static str> {
        if self.has_filename == true {
            return Err("The url has already the end with filename.");
        } else {
            self.has_filename = true;
            self.enc.push_str(&url_encode(file_name));
            //return
            Ok(())
        }
    }
    /// get encoded string
    pub fn get_enc(&self) -> String {
        // return
        self.enc.clone()
    }
}

pub fn url_encode(data: &str) -> String {
    let mut escaped = Vec::with_capacity(data.len());
    encode_into(data, &mut escaped).unwrap();
    // Encoded string is guaranteed to be ASCII
    unsafe { String::from_utf8_unchecked(escaped) }
}

#[inline]
fn encode_into<W: Write>(data: &str, mut escaped: W) -> io::Result<()> {
    for byte in data.as_bytes().iter() {
        match *byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~' => {
                escaped.write(std::slice::from_ref(byte))?;
            }
            other => {
                escaped.write(&[b'%', to_hex_digit(other >> 4), to_hex_digit(other & 15)])?;
            }
        }
    }
    Ok(())
}

#[inline]
fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

#[inline]
fn to_hex_digit(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=255 => b'A' - 10 + digit,
    }
}

pub fn url_decode(string: &str) -> Result<String, FromUrlEncodingError> {
    let mut out: Vec<u8> = Vec::with_capacity(string.len());
    let mut bytes = string.as_bytes().iter().copied();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                match bytes.next() {
                    Some(first) => match from_hex_digit(first) {
                        Some(first_val) => match bytes.next() {
                            Some(second) => match from_hex_digit(second) {
                                Some(second_val) => {
                                    out.push((first_val << 4) | second_val);
                                }
                                None => {
                                    out.push(b'%');
                                    out.push(first);
                                    out.push(second);
                                }
                            },
                            None => {
                                out.push(b'%');
                                out.push(first);
                            }
                        },
                        None => {
                            out.push(b'%');
                            out.push(first);
                        }
                    },
                    None => out.push(b'%'),
                };
            }
            other => out.push(other),
        }
    }
    String::from_utf8(out).map_err(|error| FromUrlEncodingError::Utf8CharacterError { error })
}

#[derive(Debug)]
pub enum FromUrlEncodingError {
    #[allow(dead_code)]
    UriCharacterError {
        character: char,
        index: usize,
    },
    Utf8CharacterError {
        error: FromUtf8Error,
    },
}

impl Error for FromUrlEncodingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            &FromUrlEncodingError::UriCharacterError {
                character: _,
                index: _,
            } => None,
            &FromUrlEncodingError::Utf8CharacterError { ref error } => Some(error),
        }
    }
}

impl Display for FromUrlEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &FromUrlEncodingError::UriCharacterError { character, index } => {
                write!(f, "invalid URI char [{}] at [{}]", character, index)
            }
            &FromUrlEncodingError::Utf8CharacterError { ref error } => {
                write!(f, "invalid utf8 char: {}", error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::encode;
    use super::from_hex_digit;

    #[test]
    fn it_encodes_successfully() {
        let expected = "this%20that";
        assert_eq!(expected, encode("this that"));
    }

    #[test]
    fn it_encodes_successfully_emoji() {
        let emoji_string = "👾 Exterminate!";
        let expected = "%F0%9F%91%BE%20Exterminate%21";
        assert_eq!(expected, encode(emoji_string));
    }

    #[test]
    fn it_decodes_successfully() {
        let expected = String::from("this that");
        let encoded = "this%20that";
        assert_eq!(expected, decode(encoded).unwrap());
    }

    #[test]
    fn it_decodes_successfully_emoji() {
        let expected = String::from("👾 Exterminate!");
        let encoded = "%F0%9F%91%BE%20Exterminate%21";
        assert_eq!(expected, decode(encoded).unwrap());
    }

    #[test]
    fn it_decodes_unsuccessfully_emoji() {
        let bad_encoded_string = "👾 Exterminate!";

        assert_eq!(bad_encoded_string, decode(bad_encoded_string).unwrap());
    }

    #[test]
    fn misc() {
        assert_eq!(3, from_hex_digit(b'3').unwrap());
        assert_eq!(10, from_hex_digit(b'a').unwrap());
        assert_eq!(15, from_hex_digit(b'F').unwrap());
        assert_eq!(None, from_hex_digit(b'G'));
        assert_eq!(None, from_hex_digit(9));

        assert_eq!("pureascii", encode("pureascii"));
        assert_eq!("pureascii", decode("pureascii").unwrap());
        assert_eq!("", encode(""));
        assert_eq!("", decode("").unwrap());
        assert_eq!("%00", encode("\0"));
        assert_eq!("\0", decode("\0").unwrap());
        assert!(decode("%F0%0F%91%BE%20Hello%21").is_err());
        assert_eq!("this%2that", decode("this%2that").unwrap());
        assert_eq!("this that", decode("this%20that").unwrap());
        assert_eq!("this that%", decode("this%20that%").unwrap());
        assert_eq!("this that%2", decode("this%20that%2").unwrap());
    }
}
