use strong_xml::{XmlRead, XmlWrite};
use std::str::FromStr;
use std::fmt;
use crate::{__xml_test_suites, __string_enum};

/// Size
///
/// ```rust
/// use docx::formatting::*;
///
/// let sz = Size::from(42usize);
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:spacing")]
pub struct Spacing {
    #[xml(attr = "w:after")]
    pub after: Option<Length>,

    #[xml(attr = "afterAutospacing")]
    pub after_autospacing: Option<bool>,

    #[xml(attr = "afterLines")]
    pub after_lines: Option<isize>,

    #[xml(attr = "before")]
    pub before: Option<Length>,

    #[xml(attr = "beforeAutospacing")]
    pub before_autospacing: Option<bool>,

    #[xml(attr = "beforeLines")]
    pub before_lines: Option<isize>,

    #[xml(attr = "line")]
    pub line: Option<Length>,

    #[xml(attr = "lineRule")]
    pub line_rule: Option<LineSpacingRule>,
}

/// Length in Points
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Length(pub f32);
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}pt", self.0)
    }
}


#[derive(Debug)]
pub enum LengthParseError {
    InvalidUnit,
    ParseError(<f32 as FromStr>::Err)
}

impl fmt::Display for LengthParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LengthParseError::InvalidUnit => write!(f, "invalid unit"),
            LengthParseError::ParseError(ref e) => e.fmt(f)
        }
    }
}
impl std::error::Error for LengthParseError {

}
impl FromStr for Length {
    type Err = LengthParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (val, scale) = if let Some(p) = s.find(|c: char| c.is_ascii_lowercase()) {
            let unit = &s[p..];
            let scale = match unit {
                "cm" => 25.4 * 72.,
                "mm" => 2.54 * 72.,
                "in" => 72.,
                "pt" => 1.0,
                "pc" | "pi" => 12.,
                _ => return Err(LengthParseError::InvalidUnit)
            };
            (&s[..p], scale)
        } else {
            (s, 0.05)
        };
        match <f32 as FromStr>::from_str(val) {
            Ok(value) => Ok(Length(value * scale)),
            Err(e) => Err(LengthParseError::ParseError(e)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LineSpacingRule {
    AtLeast,
    Auto,
    Exact,
}

__string_enum! {
    LineSpacingRule {
        AtLeast = "atLeast",
        Auto = "auto",
        Exact = "exact",
    }
}
