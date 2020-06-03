//! encode_decode_mod

// cargo.toml:
// percent-encoding = "2.1.0"
// anyhow = "1.0.31"

use anyhow::Error;
use core::str::FromStr;
use percent_encoding::{percent_decode_str, AsciiSet, CONTROLS};
use std::string::ToString;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn utf8_percent_encode(s: &str) -> String {
    percent_encoding::utf8_percent_encode(s, FRAGMENT).to_string()
}

// region: type with guarantee that it has been decoded

#[derive(Clone, Debug)]
pub struct PercentDecoded {
    /// private inaccessible field
    s: String,
}
impl FromStr for PercentDecoded {
    type Err = Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = percent_decode_str(s).decode_utf8()?.to_string();
        Ok(PercentDecoded { s })
    }
}
impl ToString for PercentDecoded {
    #[inline]
    fn to_string(&self) -> String {
        self.s.clone()
    }
}

// endregion: type with guarantee that it has been decoded
