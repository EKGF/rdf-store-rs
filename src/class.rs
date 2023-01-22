// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use crate::{Prefix, RDFStoreError};

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

    pub fn plural_label(&self) -> String { format!("{}s", self.local_name) } // TODO: Make this slightly smarter
}

#[cfg(test)]
mod tests {
    use {crate::class::Class, super::Prefix};

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
}
