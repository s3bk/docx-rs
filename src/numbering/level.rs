use crate::__string_enum;
use std::borrow::Cow;
use hard_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lvl")]
pub struct Level<'a> {
    #[xml(attr = "w:ilvl")]
    pub index: usize,

    #[xml(child = "w:lvlText")]
    pub level_text: LevelText<'a>,

    #[xml(child = "w:numFmt")]
    pub numbering_format: NumberingFormat,

    #[xml(child = "w:start")]
    pub start: Option<Start>,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lvlText")]
pub struct LevelText<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numFmt")]
pub struct NumberingFormat {
    #[xml(attr = "w:val")]
    pub value: NumberingFormatVal,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum NumberingFormatVal {
    Bullet,
    Decimal,
    DecimalZero,
    LowerLetter,
    LowerRoman,
    Ordinal,
    OrdinalText,
    UpperLetter,
    UpperRoman,
    IrohaFullWidth,
    None,
}

impl Default for NumberingFormatVal {
    fn default() -> Self {
        Self::Decimal
    }
}

__string_enum! {
    NumberingFormatVal {
        Bullet = "bullet",
        Decimal = "decimal",
        DecimalZero = "decimalZero",
        LowerLetter = "lowerLetter",
        LowerRoman = "lowerRoman",
        Ordinal = "ordinal",
        OrdinalText = "ordinalText",
        UpperLetter = "upperLetter",
        UpperRoman = "upperRoman",
        IrohaFullWidth = "irohaFullWidth",
        None = "none",
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:start")]
pub struct Start {
    #[xml(attr = "w:val")]
    pub value: usize,
}
