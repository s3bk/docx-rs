use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;
use crate::formatting::{IndentLevel, NumberingId};

/// Numbering Property
///
/// ```rust
/// use docx::formatting::*;
///
/// let prop = NumberingProperty::from((20, 40));
/// ```
#[derive(Clone, Copy, Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numPr")]
pub struct NumberingProperty {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: Option<NumberingId>,
    
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: Option<IndentLevel>,
}

impl From<(usize, usize)> for NumberingProperty {
    fn from(val: (usize, usize)) -> Self {
        NumberingProperty {
            id: Some(NumberingId { value: val.0 }),
            level: Some(IndentLevel { value: val.1 }),
        }
    }
}

__xml_test_suites!(
    NumberingProperty,
    NumberingProperty::default(),
    r#"<w:numPr><w:numId w:val="0"/><w:ilvl w:val="0"/></w:numPr>"#,
    NumberingProperty::from((20, 40)),
    r#"<w:numPr><w:numId w:val="20"/><w:ilvl w:val="40"/></w:numPr>"#,
);
