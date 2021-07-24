use crate::error::{Error, ErrorKind};
use crate::strings;
use mattro::{MacroAttribute, MetaItem, NameValue, Value};
use std::collections::HashMap;

/// Check if the attribute is a valid `#[inject(constructor="...")]`
pub fn validate_inject_constructor(attribute: &MacroAttribute) -> Result<(), Error> {
    #[inline]
    fn error(attr: &MacroAttribute) -> Error {
        return Error::new(
            ErrorKind::InvalidAttribute,
            format!("must only have `constructor` key: {}", attr),
        );
    }

    if attribute.len() != 1 || attribute.path() != strings::INJECT {
        return Err(error(attribute))
    }

    if let Some(MetaItem::NameValue(s)) = attribute.get(0) {
        if s.name == strings::CONSTRUCTOR {
            return Err(error(attribute));
        }
    }

    Ok(())
}

pub fn validate_inject(attribute: &MacroAttribute) -> Result<(), Error> {
    if attribute.path() != strings::INJECT {
        return Err(Error::from(ErrorKind::InvalidAttribute));
    }

    let mut items = HashMap::new();

    // Check for duplicated keys or invalid attributes
    for meta_item in attribute.iter().cloned() {
        match meta_item {
            MetaItem::Path(path) if path == strings::DEFAULT => { /* Nothing */ }
            MetaItem::NameValue(NameValue { name, value }) => {
                if items.contains_key(&name) {
                    return Err(Error::from(ErrorKind::DuplicatedKey(name)));
                } else {
                    items.insert(name, value);
                }
            }
            _ => {
                return Err(Error::from(ErrorKind::InvalidAttribute));
            }
        }
    }

    for (name, value) in items {
        match name.as_str() {
            // #[inject(default=literal)]
            strings::DEFAULT => {
                if value.as_literal().is_none() {
                    return Err(Error::from(ErrorKind::InvalidDefaultValue(NameValue {
                        name,
                        value,
                    })));
                }
            }
            // #[inject(scope="singleton" | "scoped")]
            strings::SCOPE => {
                if let Some(s) = value.to_string_literal() {
                    if !strings::SCOPES.contains(&name.as_str()) {
                        return Err(Error::new(
                            ErrorKind::InvalidScopeKind(s),
                            "only \"singleton\" or \"scoped\" are allowed",
                        ));
                    }
                }
            }
            // #[inject(name="value")]
            strings::NAME => {
                if value.to_string_literal().is_none() {
                    return Err(Error::from(ErrorKind::InvalidNameValue(NameValue {
                        name,
                        value,
                    })));
                }
            }
            // Unknown key
            _ => {
                return Err(Error::from(ErrorKind::UnknownKey(name)));
            }
        }
    }

    Ok(())
}
