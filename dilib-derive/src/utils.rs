use crate::keys;
use mattro::{MacroAttribute, MetaItem, NameValue, Value};
use std::collections::HashMap;

pub type InjectAttribute = HashMap<String, Option<Value>>;

pub enum InjectError {
    DuplicatedKey(String),
    InvalidKey(String),
    InvalidAttribute,
}

pub fn convert_to_inject_attribute_map(
    attribute: &MacroAttribute,
) -> Result<InjectAttribute, InjectError> {
    let mut items = HashMap::new();

    // helper
    let mut add_item = |name: &str, value: Option<&Value>| {
        if items.insert(name.to_owned(), value.cloned()).is_some() {
            return Err(InjectError::DuplicatedKey(name.to_owned()));
        }

        Ok(())
    };

    for meta_item in attribute.iter() {
        match meta_item {
            MetaItem::Path(path) => {
                if path == keys::DEFAULT {
                    add_item(path, None)?;
                }
            }
            MetaItem::NameValue(NameValue { name, value }) => match name.as_str() {
                keys::DEFAULT => add_item(name, Some(value))?,
                keys::SCOPE => add_item(name, Some(value))?,
                keys::NAME => add_item(name, Some(value))?,
                _ => return Err(InjectError::InvalidKey(name.clone())),
            },
            _ => {
                return Err(InjectError::InvalidAttribute);
            }
        }
    }

    Ok(items)
}
