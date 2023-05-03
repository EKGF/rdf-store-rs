// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use crate::{Literal, Prefix, RDFStoreError};

#[derive(Debug, Clone)]
pub struct Class {
    pub prefix:     Prefix,
    pub local_name: String,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.prefix.name.as_str(),
            self.local_name.as_str()
        )
    }
}

impl Class {
    pub fn declare(prefix: Prefix, local_name: &str) -> Self {
        Self { prefix, local_name: local_name.to_string() }
    }

    pub fn as_iri(&self) -> Result<iref::IriBuf, RDFStoreError> {
        let iri = iref::IriBuf::new(format!("{}{}", self.prefix.iri, self.local_name).as_str())?;
        Ok(iri)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn display_turtle<'a>(&'a self) -> impl std::fmt::Display + 'a {
        struct TurtleClass<'a>(&'a Class);
        impl<'a> std::fmt::Display for TurtleClass<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}{}", self.0.prefix.name, self.0.local_name)
            }
        }
        TurtleClass(self)
    }

    pub fn plural_label(&self) -> String { format!("{}s", self.local_name) }

    // TODO: Make this slightly smarter

    pub fn is_literal(&self, literal: &Literal) -> bool {
        if let Some(that_iri) = literal.as_iri() {
            if let Ok(this_iri) = self.as_iri() {
                that_iri == this_iri.as_iri()
            } else {
                let iri = self.to_string();
                literal.to_string() == iri
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Prefix,
        crate::{class::Class, DataType, Literal},
    };

    #[test]
    fn test_a_class_01() {
        let prefix = Prefix::declare(
            "test:",
            iref::Iri::new("https://whatever.com/test#").unwrap(),
        );
        let class = Class::declare(prefix, "SomeClass");
        let s = format!("{:}", class);
        assert_eq!(s, "test:SomeClass")
    }

    #[test]
    fn test_a_class_02() {
        let prefix = Prefix::declare(
            "test:",
            iref::Iri::new("https://whatever.com/test#").unwrap(),
        );
        let class = Class::declare(prefix, "SomeClass");
        let s = format!("{}", class.as_iri().unwrap());
        assert_eq!(s, "https://whatever.com/test#SomeClass");
    }

    #[test]
    fn test_is_literal() {
        let prefix = Prefix::declare(
            "test:",
            iref::Iri::new("https://whatever.com/test#").unwrap(),
        );
        let class = Class::declare(prefix, "SomeClass");
        let literal = Literal::from_type_and_buffer(
            DataType::AnyUri,
            "https://whatever.com/test#SomeClass",
            None,
        )
        .unwrap();
        assert!(literal.is_some());
        assert_eq!(
            class.as_iri().unwrap().as_str(),
            "https://whatever.com/test#SomeClass"
        );
        assert!(class.is_literal(&literal.unwrap()))
    }
}
