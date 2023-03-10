// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

mod value;

pub use value::LiteralValue;

use {
    crate::{
        DataType,
        RDFStoreError::{self, Unknown},
        Term,
    },
    iref::{Iri, IriBuf},
    serde::{Serialize, Serializer},
    std::{
        fmt::{Debug, Display, Formatter},
        mem::ManuallyDrop,
        str::FromStr,
    },
};

/// From [RDF 1.1 Concepts and Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal):
///
/// Literals are used for values such as strings, numbers, and dates.
/// A literal in an RDF graph consists of two or three elements:
///
/// 1. a lexical form, being a Unicode string, which SHOULD be in
///    [Normal Form C](http://www.unicode.org/reports/tr15/)
///
/// 2. a datatype IRI, being an IRI identifying a datatype that determines
///    how the lexical form maps to a literal value, and if and only if the
///    datatype IRI is http://www.w3.org/1999/02/22-rdf-syntax-ns#langString,
///    a non-empty language tag as defined by [BCP47](https://www.rfc-editor.org/info/bcp47).
///    The language tag MUST be well-formed according to
///    section 2.2.9 of [BCP47](https://www.rfc-editor.org/info/bcp47).
#[derive(Default)]
pub struct Literal {
    pub data_type: DataType,
    lexical_value: LiteralValue,
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        let data_type = self.data_type;
        if data_type != other.data_type {
            return false
        }
        unsafe {
            if data_type.is_iri() {
                self.lexical_value.iri == other.lexical_value.iri
            } else if data_type.is_string() {
                self.lexical_value.string == other.lexical_value.string
            } else if data_type.is_boolean() {
                self.lexical_value.boolean == other.lexical_value.boolean
            } else if data_type.is_signed_integer() {
                self.lexical_value.signed_integer == other.lexical_value.signed_integer
            } else if data_type.is_unsigned_integer() {
                self.lexical_value.unsigned_integer == other.lexical_value.unsigned_integer
            } else if data_type.is_blank_node() {
                self.lexical_value.blank_node == other.lexical_value.blank_node
            } else if data_type.is_decimal() {
                self.lexical_value.string == other.lexical_value.string
            } else {
                panic!("Cannot compare, unimplemented datatype {data_type:?}")
            }
        }
    }
}

impl Eq for Literal {}

impl std::hash::Hash for Literal {
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher {
        let data_type = self.data_type;
        data_type.hash(state);
        unsafe {
            if data_type.is_iri() {
                self.lexical_value.iri.hash(state)
            } else if data_type.is_string() {
                self.lexical_value.string.hash(state)
            } else if data_type.is_blank_node() {
                self.lexical_value.blank_node.hash(state)
            } else if data_type.is_boolean() {
                self.lexical_value.boolean.hash(state)
            } else if data_type.is_signed_integer() {
                self.lexical_value.signed_integer.hash(state)
            } else if data_type.is_unsigned_integer() {
                self.lexical_value.unsigned_integer.hash(state)
            } else if data_type.is_decimal() {
                self.lexical_value.string.hash(state)
            } else if data_type.is_duration() {
                self.lexical_value.string.hash(state)
            } else {
                panic!("Cannot hash, unimplemented datatype {data_type:?}")
            }
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data_type = self.data_type;
        write!(f, "Literal({:?},", data_type)?;
        unsafe {
            if data_type.is_iri() {
                write!(f, "<{}>)", self.lexical_value.iri.as_str())?
            } else if data_type.is_string() {
                write!(f, "\"{}\"", self.lexical_value.string.as_str())?
            } else if data_type.is_blank_node() {
                write!(f, "_:{}", self.lexical_value.blank_node.as_str())?
            } else if data_type.is_boolean() {
                write!(f, "{}", self.lexical_value.boolean)?
            } else if data_type.is_signed_integer() {
                write!(f, "{}", self.lexical_value.signed_integer)?
            } else if data_type.is_unsigned_integer() {
                write!(f, "{}", self.lexical_value.unsigned_integer)?
            } else if data_type.is_decimal() {
                write!(f, "{}", self.lexical_value.string.as_str())?
            } else if data_type.is_duration() ||
                data_type.is_date_time() ||
                data_type.is_date() ||
                data_type.is_date_time_stamp()
            {
                write!(f, "{}", self.lexical_value.string.as_str())?
            } else {
                panic!("Cannot format, unimplemented datatype {data_type:?}")
            }
        }
        write!(f, ")")
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data_type.is_iri() {
            write!(f, "<{}>", self.as_iri().unwrap().as_str())
        } else if self.data_type.is_blank_node() {
            write!(f, "_:{}", self.as_string().unwrap().as_str())
        } else if self.data_type.is_string() {
            if let Some(strng) = self.as_string() {
                write!(f, "\"{}\"", strng.as_str())
            } else {
                write!(f, "ERROR, could not convert to String")
            }
        } else {
            if let Some(strng) = self.as_string() {
                write!(f, "{} ({:?})", strng.as_str(), self.data_type)
            } else {
                write!(
                    f,
                    "ERROR, could not convert to String ({:?})",
                    self.data_type
                )
            }
        }
    }
}

impl Clone for Literal {
    // noinspection RsUnreachableCode
    fn clone(&self) -> Self {
        if self.data_type.is_iri() {
            if let Some(ref iri) = self.as_iri() {
                Literal {
                    data_type: self.data_type,
                    lexical_value:     LiteralValue::new_iri(iri),
                }
            } else {
                todo!("the situation where the iri in a lexical value is empty")
            }
        } else if self.data_type.is_blank_node() {
            if let Some(blank_node) = self.as_str() {
                Literal::new_blank_node_with_datatype(blank_node, self.data_type).unwrap()
            } else {
                todo!("the situation where the blank_node in a lexical value is empty")
            }
        } else if self.data_type.is_string() {
            if let Some(str) = self.as_str() {
                Literal::new_string_with_datatype(str, self.data_type).unwrap()
            } else {
                todo!("the situation where the string in a lexical value is empty")
            }
        } else if self.data_type.is_boolean() {
            if let Some(boolean) = self.as_boolean() {
                Literal::new_boolean_with_datatype(boolean, self.data_type).unwrap()
            } else {
                todo!("the situation where the boolean in a lexical value is not a boolean")
            }
        } else if self.data_type.is_date_time() {
            if let Some(date_time) = self.as_date_time() {
                Literal::new_date_time_with_datatype(date_time, self.data_type).unwrap()
            } else {
                todo!("the situation where the boolean in a lexical value is not a boolean")
            }
        } else if self.data_type.is_signed_integer() {
            if let Some(long) = self.as_signed_long() {
                Literal::new_signed_integer_with_datatype(long, self.data_type).unwrap()
            } else {
                todo!("the situation where the signed integer value is not a long")
            }
        } else if self.data_type.is_unsigned_integer() {
            if let Some(long) = self.as_unsigned_long() {
                Literal::new_unsigned_integer_with_datatype(long, self.data_type).unwrap()
            } else {
                todo!("the situation where the unsigned integer value is not a long")
            }
        } else if self.data_type.is_decimal() {
            if let Some(decimal) = self.as_decimal() {
                Literal::new_decimal_with_datatype(decimal, self.data_type).unwrap()
            } else {
                todo!("the situation where the decimal value is not a decimal")
            }
        } else if self.data_type.is_duration() {
            if let Some(duration) = self.as_duration() {
                Literal::new_duration_with_datatype(duration, self.data_type).unwrap()
            } else {
                todo!("the situation where the duration value is not a duration")
            }
        } else {
            todo!(
                "dealing with other datatypes: {:?}",
                self.data_type
            )
        }
    }
}

impl Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let data_type = self.data_type;
        unsafe {
            if data_type.is_iri() {
                serializer.serialize_str(self.lexical_value.iri.as_str())
            } else if data_type.is_string() {
                serializer.serialize_str(self.lexical_value.string.as_str())
            } else if data_type.is_blank_node() {
                serializer.serialize_str(self.lexical_value.blank_node.as_str())
            } else if data_type.is_boolean() {
                serializer.serialize_bool(self.lexical_value.boolean)
            } else if data_type.is_signed_integer() {
                serializer.serialize_i64(self.lexical_value.signed_integer)
            } else if data_type.is_unsigned_integer() {
                serializer.serialize_u64(self.lexical_value.unsigned_integer)
            } else if data_type.is_decimal() {
                serializer.serialize_str(self.lexical_value.string.as_str())
            } else if data_type.is_duration() ||
                data_type.is_date_time() ||
                data_type.is_date() ||
                data_type.is_date_time_stamp()
            {
                serializer.serialize_str(self.lexical_value.string.as_str())
            } else {
                panic!("Cannot serialize, unimplemented datatype {data_type:?}")
            }
        }
    }
}

impl Literal {
    pub fn as_term(&self) -> Term {
        match self.data_type {
            DataType::IriReference | DataType::AnyUri => Term::Iri(self.clone()),
            DataType::BlankNode => Term::BlankNode(self.clone()),
            _ => Term::Literal(self.clone()),
        }
    }

    pub fn as_iri(&self) -> Option<Iri> {
        if self.data_type.is_iri() {
            Some(unsafe { self.lexical_value.iri.as_iri() })
        } else {
            None
        }
    }

    pub fn as_local_name(&self) -> Option<String> {
        self.as_iri().as_ref().and_then(|iri| {
            let iri_str = iri.as_str();
            match fancy_regex::Regex::new(r#"(?:.*)[#/](.*)"#) {
                Ok(re) => {
                    if let Ok(Some(captures)) = re.captures(iri_str) {
                        if let Some(mat) = captures.get(1) {
                            Some(String::from(mat.as_str()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Err(_err) => {
                    tracing::error!("Literal::as_local_name failed with iri: {iri_str}");
                    None
                },
            }
        })
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.data_type.is_iri() {
            unsafe { Some(self.lexical_value.iri.as_str()) }
        } else if self.data_type.is_string() {
            unsafe { Some(self.lexical_value.string.as_str()) }
        } else if self.data_type.is_signed_integer() {
            None
        } else if self.data_type.is_unsigned_integer() {
            None
        } else if self.data_type.is_blank_node() {
            unsafe { Some(self.lexical_value.blank_node.as_str()) }
        } else if self.data_type.is_boolean() {
            unsafe {
                if self.lexical_value.boolean {
                    Some("true")
                } else {
                    Some("false")
                }
            }
        } else if self.data_type.is_decimal() {
            unsafe { Some(self.lexical_value.string.as_str()) }
        } else if self.data_type.is_duration() {
            unsafe { Some(self.lexical_value.string.as_str()) }
        } else if self.data_type.is_date_time() {
            unsafe { Some(self.lexical_value.string.as_str()) }
        } else {
            panic!("Data type {:?} not yet supported", self.data_type);
        }
    }

    pub fn as_string(&self) -> Option<String> { self.as_str().map(|v| v.to_owned()) }

    pub fn as_boolean(&self) -> Option<bool> {
        match self.data_type {
            DataType::Boolean => Some(unsafe { self.lexical_value.boolean }),
            _ => None,
        }
    }

    pub fn as_signed_long(&self) -> Option<i64> {
        if self.data_type.is_signed_integer() {
            Some(unsafe { self.lexical_value.signed_integer })
        } else {
            None
        }
    }

    pub fn as_unsigned_long(&self) -> Option<u64> {
        if self.data_type.is_unsigned_integer() {
            Some(unsafe { self.lexical_value.unsigned_integer })
        } else {
            None
        }
    }

    pub fn as_date_time(&self) -> Option<&str> {
        match self.data_type {
            DataType::DateTime => Some(unsafe { &self.lexical_value.string }),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<&str> {
        match self.data_type {
            DataType::Decimal => Some(unsafe { &self.lexical_value.string }),
            _ => None,
        }
    }

    pub fn as_duration(&self) -> Option<&str> {
        match self.data_type {
            DataType::Duration => Some(unsafe { &self.lexical_value.string }),
            _ => None,
        }
    }

    pub fn from_type_and_c_buffer(
        data_type: DataType,
        buffer: &[u8],
    ) -> Result<Option<Literal>, RDFStoreError> {
        let str_buffer = std::ffi::CStr::from_bytes_until_nul(buffer)
            .map_err(|err| {
                tracing::error!("Cannot read buffer: {err:?}");
                Unknown // TODO
            })?
            .to_str()
            .map_err(|err| {
                tracing::error!("Cannot convert buffer to string: {err:?}");
                Unknown // TODO
            })?;
        Self::from_type_and_buffer(data_type, str_buffer)
    }

    pub fn from_type_and_buffer(
        data_type: DataType,
        buffer: &str,
    ) -> Result<Option<Literal>, RDFStoreError> {
        match data_type {
            DataType::AnyUri | DataType::IriReference => {
                let iri = IriBuf::from_str(buffer)?;
                Ok(Some(Literal::new_iri_with_datatype(
                    &iri.as_iri(),
                    data_type,
                )?))
            },
            DataType::BlankNode => {
                Ok(Some(Literal::new_blank_node_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Boolean => {
                match buffer {
                    "true" | "false" => {
                        Ok(Some(Literal::new_boolean_with_datatype(
                            buffer.starts_with("true"),
                            data_type,
                        )?))
                    },
                    _ => Err(RDFStoreError::UnknownNTriplesValue { value: buffer.to_string() }),
                }
            },
            DataType::String | DataType::PlainLiteral => {
                Ok(Some(Literal::new_string_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::DateTime => {
                Ok(Some(Literal::new_date_time_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Int |
            DataType::Integer |
            DataType::NegativeInteger |
            DataType::NonPositiveInteger |
            DataType::Long |
            DataType::Short => {
                let signed_integer: i64 = buffer.parse().unwrap(); // TODO: Remove unwrap
                Ok(Some(Literal::new_signed_integer_with_datatype(
                    signed_integer,
                    data_type,
                )?))
            },
            DataType::PositiveInteger |
            DataType::NonNegativeInteger |
            DataType::UnsignedByte |
            DataType::UnsignedInt |
            DataType::UnsignedShort |
            DataType::UnsignedLong => {
                let unsigned_integer: u64 = buffer.parse().unwrap(); // TODO: Remove unwrap
                Ok(Some(Literal::new_unsigned_integer_with_datatype(
                    unsigned_integer,
                    data_type,
                )?))
            },
            DataType::Decimal => {
                Ok(Some(Literal::new_decimal_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Duration => {
                Ok(Some(Literal::new_duration_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::UnboundValue => Ok(None),
            _ => {
                tracing::warn!("Unsupported datatype: {data_type:?} value={buffer}");
                Err(Unknown)
            },
        }
    }

    pub fn from_iri(iri: &Iri) -> Result<Self, RDFStoreError> {
        Ok(Literal {
            data_type: DataType::IriReference,
            lexical_value:     LiteralValue { iri: ManuallyDrop::new(IriBuf::from(iri)) },
        })
    }

    pub fn new_plain_literal_string(str: &str) -> Result<Self, RDFStoreError> {
        Self::new_string_with_datatype(str, DataType::PlainLiteral)
    }

    pub fn new_plain_literal_boolean(boolean: bool) -> Result<Self, RDFStoreError> {
        Self::new_string_with_datatype(
            boolean.to_string().as_str(),
            DataType::PlainLiteral,
        )
    }

    pub fn new_string_with_datatype(str: &str, data_type: DataType) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_string(),
            "{data_type:?} is not a string type"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_string(str) })
    }

    pub fn new_date_time_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_date_time(),
            "{data_type:?} is not a dateTime"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_string(str) })
    }

    pub fn new_decimal_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_decimal(),
            "{data_type:?} is not a decimal"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_string(str) })
    }

    pub fn new_duration_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_duration(),
            "{data_type:?} is not a duration"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_string(str) })
    }

    pub fn new_iri_from_string_with_datatype(
        iri_string: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        let iri = IriBuf::from_str(iri_string)?;
        Self::new_iri_with_datatype(&iri.as_iri(), data_type)
    }

    pub fn new_iri_with_datatype(iri: &Iri, data_type: DataType) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_iri(),
            "{data_type:?} is not an IRI type"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_iri(iri) })
    }

    pub fn new_blank_node_with_datatype(
        id: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_blank_node(),
            "{data_type:?} is not a blank node type"
        );
        Ok(Literal { data_type, lexical_value: LiteralValue::new_blank_node(id) })
    }

    pub fn new_boolean(boolean: bool) -> Result<Self, RDFStoreError> {
        Self::new_boolean_with_datatype(boolean, DataType::Boolean)
    }

    pub fn new_boolean_from_string(boolean_string: &str) -> Result<Self, RDFStoreError> {
        Self::new_boolean_from_string_with_datatype(boolean_string, DataType::Boolean)
    }

    pub fn new_boolean_from_string_with_datatype(
        boolean_string: &str,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        match boolean_string {
            "true" => Self::new_boolean_with_datatype(true, data_type),
            "false" => Self::new_boolean_with_datatype(false, data_type),
            &_ => {
                Err(RDFStoreError::UnknownValueForDataType {
                    data_type,
                    value: boolean_string.to_string(),
                })
            },
        }
    }

    pub fn new_boolean_with_datatype(
        boolean: bool,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_boolean(),
            "{data_type:?} is not a boolean type"
        );
        Ok(Literal {
            data_type,
            lexical_value: LiteralValue::new_boolean(boolean),
        })
    }

    pub fn new_signed_integer(signed_integer: i64) -> Result<Self, RDFStoreError> {
        if signed_integer >= 0 {
            Self::new_unsigned_integer(signed_integer as u64)
        } else {
            Self::new_signed_integer_with_datatype(signed_integer, DataType::NegativeInteger)
        }
    }

    pub fn new_signed_integer_with_datatype(
        signed_integer: i64,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_signed_integer(),
            "{data_type:?} is not an signed integer type"
        );
        Ok(Literal {
            data_type,
            lexical_value: LiteralValue::new_signed_integer(signed_integer),
        })
    }

    pub fn new_unsigned_integer(unsigned_integer: u64) -> Result<Self, RDFStoreError> {
        Self::new_unsigned_integer_with_datatype(unsigned_integer, DataType::PositiveInteger)
    }

    pub fn new_unsigned_integer_with_datatype(
        unsigned_integer: u64,
        data_type: DataType,
    ) -> Result<Self, RDFStoreError> {
        assert!(
            &data_type.is_unsigned_integer(),
            "{data_type:?} is not an unsigned integer type"
        );
        Ok(Literal {
            data_type,
            lexical_value: LiteralValue::new_unsigned_integer(unsigned_integer),
        })
    }

    pub fn display_turtle<'a, 'b>(&'a self) -> impl std::fmt::Display + 'a + 'b
    where 'a: 'b {
        struct TurtleLexVal<'b>(&'b Literal);
        impl<'b> std::fmt::Display for TurtleLexVal<'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let data_type = self.0.data_type;
                unsafe {
                    if data_type.is_iri() {
                        write!(f, "<{}>", self.0.lexical_value.iri.as_str())?
                    } else if data_type.is_string() {
                        write!(f, "\"{}\"", self.0.lexical_value.string.as_str())?
                    } else if data_type.is_blank_node() {
                        write!(f, "_:{}", self.0.lexical_value.blank_node.as_str())?
                    } else if data_type.is_boolean() {
                        write!(f, "{}", self.0.lexical_value.boolean)?
                    } else if data_type.is_signed_integer() {
                        write!(f, "{}", self.0.lexical_value.signed_integer)?
                    } else if data_type.is_unsigned_integer() {
                        write!(f, "{}", self.0.lexical_value.unsigned_integer)?
                    } else if data_type.is_date_time() {
                        write!(
                            f,
                            "\"{}\"^^xsd:dateTime",
                            self.0.lexical_value.string.as_str()
                        )?
                    } else if data_type.is_decimal() {
                        write!(f, "{}", self.0.lexical_value.string.as_str())?
                    } else if data_type.is_duration() {
                        write!(
                            f,
                            "\"{}\"^^xsd:duration",
                            self.0.lexical_value.string.as_str()
                        )?
                    } else {
                        panic!("Cannot format for turtle, unimplemented datatype {data_type:?}")
                    }
                }
                Ok(())
            }
        }
        TurtleLexVal(self)
    }

    pub fn display_json<'a, 'b>(&'a self) -> impl std::fmt::Display + 'a + 'b
    where 'a: 'b {
        struct JsonLexVal<'b>(&'b Literal);
        impl<'b> Display for JsonLexVal<'b> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let data_type = self.0.data_type;
                unsafe {
                    if data_type.is_iri() {
                        write!(f, "\"{}\"", self.0.lexical_value.iri.as_str())?
                    } else if data_type.is_string() {
                        write!(
                            f,
                            "\"{}\"",
                            self.0.lexical_value.string.replace("\"", "\\\"").as_str()
                        )?
                    } else if data_type.is_blank_node() {
                        write!(f, "\"_:{}\"", self.0.lexical_value.blank_node.as_str())?
                    } else if data_type.is_boolean() {
                        write!(f, "{}", self.0.lexical_value.boolean)?
                    } else if data_type.is_signed_integer() {
                        write!(f, "{}", self.0.lexical_value.signed_integer)?
                    } else if data_type.is_unsigned_integer() {
                        write!(f, "{}", self.0.lexical_value.unsigned_integer)?
                    } else if data_type.is_date_time() {
                        write!(f, "\"{}\"", self.0.lexical_value.string.as_str())?
                    } else if data_type.is_decimal() {
                        write!(f, "{}", self.0.lexical_value.string.as_str())?
                    } else if data_type.is_duration() {
                        write!(f, "\"{}\"", self.0.lexical_value.string.as_str())?
                    } else {
                        panic!("Cannot format for JSON, unimplemented datatype {data_type:?}")
                    }
                }
                Ok(())
            }
        }
        JsonLexVal(self)
    }
}

impl FromStr for Literal {
    type Err = RDFStoreError;

    fn from_str(str: &str) -> Result<Self, Self::Err> { Self::new_plain_literal_string(str) }
}

#[cfg(test)]
mod tests {
    use {
        crate::{Literal, RDFStoreError},
        iref::IriBuf,
        std::str::FromStr,
    };

    #[test]
    fn test_as_local_name_01() -> Result<(), RDFStoreError> {
        let val = Literal::from_iri(&IriBuf::from_str("https://whatever.kg/id/abc")?.as_iri());
        assert!(val.is_ok());
        let val = val.unwrap();
        let name = val.as_local_name();
        assert!(name.is_some());
        let name = name.unwrap();
        assert_eq!(name, "abc");
        Ok(())
    }

    #[test]
    fn test_as_local_name_02() -> Result<(), RDFStoreError> {
        let val = Literal::from_iri(&IriBuf::from_str("https://whatever.kg/id#abc")?.as_iri());
        assert!(val.is_ok());
        let val = val.unwrap();
        let name = val.as_local_name();
        assert!(name.is_some());
        let name = name.unwrap();
        assert_eq!(name, "abc");
        Ok(())
    }
}
