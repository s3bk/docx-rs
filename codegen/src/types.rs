use regex::Regex;

#[derive(Debug)]
pub(crate) struct Attribute {
  pub key: String,
  pub value: String,
}

#[derive(Debug)]
pub(crate) struct Field {
  pub name: String,
  pub ty: String,
  pub attrs: Attribute,
  pub is_vec: bool,
  pub is_option: bool,
}

pub(crate) enum FieldType {
  String,
  Cow,
  Slices,
  Others(String),
}

impl Field {
  pub fn get_ty(&self) -> FieldType {
    let cow_re = Regex::new(r"Cow<'(\w+), str>").unwrap();
    let str_re = Regex::new(r"&'(\w) str").unwrap();

    if self.ty == "String" {
      FieldType::String
    } else if cow_re.is_match(&self.ty) {
      FieldType::Cow
    } else if str_re.is_match(&self.ty) {
      FieldType::Slices
    } else {
      FieldType::Others(self.ty.clone())
    }
  }
}

#[derive(Debug)]
pub(crate) struct Structure {
  pub name: String,
  pub fields: Vec<Field>,
  pub attrs: Attribute,
}

impl Structure {
  pub fn find_field(&self, key: &'static str) -> &Field {
    self.fields.iter().find(|f| f.attrs.key == key).unwrap()
  }

  pub fn filter_field(&self, key: &'static str) -> Vec<&Field> {
    self
      .fields
      .iter()
      .filter(|f| f.attrs.key == key)
      .collect::<Vec<_>>()
  }
}

pub(crate) fn parse_struct(struct_str: String) -> Structure {
  let struct_re =
    Regex::new(r#"#\[xml\((?P<key>.+) = "(?P<value>.+)"\)\]\n(:?pub )?struct (?P<name>.+) \{"#)
      .unwrap();

  let filed_re =
    Regex::new(r#"#\[xml\((?P<key>[:\w]+)(:? = )?(:?"(?P<value>.+)")?\)\]\n\s+(:?pub )(?P<name>.+): (?P<ty>.+),"#)
      .unwrap();

  let option_re = Regex::new(r#"Option<(?P<ty>.+)>"#).unwrap();

  let vec_re = Regex::new(r#"Vec<(?P<ty>.+)>"#).unwrap();

  let fields: Vec<_> = filed_re
    .captures_iter(&struct_str)
    .map(|caps| {
      let mut is_vec = false;
      let mut is_option = false;
      let mut ty = caps["ty"].to_string();

      if let Some(caps) = option_re.captures(&ty.clone()) {
        is_option = true;
        ty = caps["ty"].to_string();
      }

      if let Some(caps) = vec_re.captures(&ty.clone()) {
        is_vec = true;
        ty = caps["ty"].to_string();
      }

      Field {
        ty,
        is_vec,
        is_option,
        name: caps["name"].to_string(),
        attrs: Attribute {
          key: caps["key"].to_string(),
          value: caps
            .name("value")
            .map(|m| m.as_str().to_string())
            .unwrap_or("".to_string()),
        },
      }
    }).collect();

  let caps = struct_re.captures(&struct_str).unwrap();

  Structure {
    name: caps["name"].to_string(),
    fields,
    attrs: Attribute {
      key: caps["key"].to_string(),
      value: caps
        .name("value")
        .map(|m| m.as_str().to_string())
        .unwrap_or("".to_string()),
    },
  }
}