use std::str::FromStr;
// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------
use iref::{Iri, IriBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prefix {
    /// assumed to end with ':'
    pub name: String,
    /// assumed to end with either '/' or '#'
    pub iri:  IriBuf,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name.as_str(), self.iri.as_str())
    }
}

impl Prefix {
    pub fn declare<'a, Base: Into<Iri<'a>>>(name: &str, iri: Base) -> Self {
        let iri = iri.into();
        match iri.as_str().chars().last() {
            Some('/') | Some('#') => {
                Self {
                    name: name.to_string(),
                    iri:  IriBuf::from(iri),
                }
            },
            _ => {
                Self {
                    name: name.to_string(),
                    iri:  IriBuf::from_string(format!("{}/", iri)).unwrap(),
                }
            },
        }
    }

    pub fn declare_from_str(name: &str, iri: &str) -> Self {
        Self::declare(name, Iri::from_str(iri).unwrap())
    }

    /// Return an identifier based on the current namespace IRI and the given local name
    /// within that namespace.
    pub fn with_local_name(&self, name: &str) -> Result<IriBuf, iref::Error> {

        let iri_str = match *self.iri.as_bytes().last().unwrap() as char {
            '/' | '#' => format!("{}{name}", self.iri.as_str()),
            _ => panic!("{} does not end with either / or #", self.iri.as_str())
        };

        IriBuf::from_str(iri_str.as_str())
    }

    #[cfg(feature = "rdftk_support")]
    pub fn as_rdftk_iri_ref(&self) -> Result<rdftk_iri::IRIRef, rdftk_iri::error::Error> {
        Ok(rdftk_iri::IRIRef::new(self.as_rdftk_iri()?))
    }

    #[cfg(feature = "rdftk_support")]
    pub fn as_rdftk_iri(&self) -> Result<rdftk_iri::IRI, rdftk_iri::error::Error> {
        use std::str::FromStr;
        rdftk_iri::IRI::from_str(self.iri.as_str())
    }
}

#[cfg(test)]
mod tests {
    use {iref::Iri, super::Prefix};

    #[test_log::test]
    fn test_a_prefix() -> Result<(), iref::Error> {
        let prefix = Prefix::declare(
            "test:",
            Iri::new("http://whatever.kom/test#").unwrap(),
        );
        let x = prefix.with_local_name("abc")?;

        assert_eq!(x.as_str(), "http://whatever.kom/test#abc");
        Ok(())
    }

    #[test_log::test]
    fn test_b_prefix() -> Result<(), iref::Error> {
        let prefix = Prefix::declare(
            "test:",
            Iri::new("http://whatever.kom/test/").unwrap(),
        );
        let x = prefix.with_local_name("abc")?;

        assert_eq!(x.as_str(), "http://whatever.kom/test/abc");
        Ok(())
    }

}
