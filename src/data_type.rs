// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------
use {
    crate::{RDFStoreError, RDFStoreError::UnknownDataType},
    num_enum::TryFromPrimitive,
    phf::phf_map,
    serde::Serialize,
};

static DATA_TYPE_MAP: phf::Map<&'static str, DataType> = phf_map! {
    "Unbound Value" => DataType::UnboundValue,
    "Blank Node" => DataType::BlankNode,
    "IRI Reference" => DataType::IriReference,
    "http://www.w3.org/2000/01/rdf-schema#Literal" => DataType::Literal,
    "http://www.w3.org/2001/XMLSchema#anyURI" => DataType::AnyUri,
    "http://www.w3.org/2001/XMLSchema#boolean" => DataType::Boolean,
    "http://www.w3.org/2001/XMLSchema#byte" => DataType::Byte,
    "http://www.w3.org/2001/XMLSchema#date" => DataType::Date,
    "http://www.w3.org/2001/XMLSchema#dateTime" => DataType::DateTime,
    "http://www.w3.org/2001/XMLSchema#dateTimeStamp" => DataType::DateTimeStamp,
    "http://www.w3.org/2001/XMLSchema#gDay" => DataType::Day,
    "http://www.w3.org/2001/XMLSchema#dayTimeDuration" => DataType::DayTimeDuration,
    "http://www.w3.org/2001/XMLSchema#decimal" => DataType::Decimal,
    "http://www.w3.org/2001/XMLSchema#double" => DataType::Double,
    "http://www.w3.org/2001/XMLSchema#duration" => DataType::Duration,
    "http://www.w3.org/2001/XMLSchema#float" => DataType::Float,
    "http://www.w3.org/2001/XMLSchema#int" => DataType::Int,
    "http://www.w3.org/2001/XMLSchema#integer" => DataType::Integer,
    "http://www.w3.org/2001/XMLSchema#long" => DataType::Long,
    "http://www.w3.org/2001/XMLSchema#gMonth" => DataType::Month,
    "http://www.w3.org/2001/XMLSchema#gMonthDay" => DataType::MonthDay,
    "http://www.w3.org/2001/XMLSchema#negativeInteger" => DataType::NegativeInteger,
    "http://www.w3.org/2001/XMLSchema#nonNegativeInteger" => DataType::NonNegativeInteger,
    "http://www.w3.org/2001/XMLSchema#nonPositiveInteger" => DataType::NonPositiveInteger,
    "http://www.w3.org/2001/XMLSchema#short" => DataType::Short,
    "http://www.w3.org/2001/XMLSchema#string" => DataType::String,
    "http://www.w3.org/2001/XMLSchema#time" => DataType::Time,
    "http://www.w3.org/2001/XMLSchema#unsignedByte" => DataType::UnsignedByte,
    "http://www.w3.org/2001/XMLSchema#unsignedInt" => DataType::UnsignedInt,
    "http://www.w3.org/2001/XMLSchema#unsignedLong" => DataType::UnsignedLong,
    "http://www.w3.org/2001/XMLSchema#unsignedShort" => DataType::UnsignedShort,
    "http://www.w3.org/2001/XMLSchema#gYear" => DataType::Year,
    "http://www.w3.org/2001/XMLSchema#gYearMonth" => DataType::YearMonth,
    "http://www.w3.org/2001/XMLSchema#yearMonthDuration" => DataType::YearMonthDuration,
};

/// The XSD DataType of a given [`Literal`].
/// See also <https://docs.oxfordsemantic.tech/_javadoc/tech/oxfordsemantic/jrdfox/logic/Datatype.html>.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, TryFromPrimitive, Serialize)]
#[repr(u8)]
pub enum DataType {
    /// INVALID_DATATYPE
    UnboundValue       = 0,
    /// BLANK_NODE
    BlankNode          = 1,
    /// IRI_REFERENCE
    IriReference       = 2,
    /// RDFS_LITERAL
    Literal            = 3,
    /// XSD_ANY_URI
    AnyUri             = 4,
    /// XSD_STRING
    String             = 5,
    /// RDF_PLAIN_LITERAL
    PlainLiteral       = 6,
    /// XSD_BOOLEAN
    Boolean            = 7,
    /// XSD_DATE_TIME
    DateTime           = 8,
    /// XSD_DATE_TIME_STAMP
    DateTimeStamp      = 9,
    /// XSD_TIME
    Time               = 10,
    /// XSD_DATE
    Date               = 11,
    /// XSD_G_YEAR_MONTH
    YearMonth          = 12,
    /// XSD_G_YEAR
    Year               = 13,
    /// XSD_G_MONTH_DAY
    MonthDay           = 14,
    /// XSD_G_DAY
    Day                = 15,
    /// XSD_G_MONTH
    Month              = 16,
    /// XSD_DURATION
    Duration           = 17,
    /// XSD_YEAR_MONTH_DURATION
    YearMonthDuration  = 18,
    /// XSD_DAY_TIME_DURATION
    DayTimeDuration    = 19,
    /// XSD_DOUBLE
    Double             = 20,
    /// XSD_FLOAT
    Float              = 21,
    /// XSD_DECIMAL
    Decimal            = 22,
    /// XSD_INTEGER
    Integer            = 23,
    /// XSD_NON_NEGATIVE_INTEGER
    NonNegativeInteger = 24,
    /// XSD_NON_POSITIVE_INTEGER
    NonPositiveInteger = 25,
    /// XSD_NEGATIVE_INTEGER
    NegativeInteger    = 26,
    /// XSD_POSITIVE_INTEGER
    PositiveInteger    = 27,
    /// XSD_LONG
    Long               = 28,
    /// XSD_INT
    Int                = 29,
    /// XSD_SHORT
    Short              = 30,
    /// XSD_BYTE
    Byte               = 31,
    /// XSD_UNSIGNED_LONG
    UnsignedLong       = 32,
    /// XSD_UNSIGNED_INT
    UnsignedInt        = 33,
    /// XSD_UNSIGNED_SHORT
    UnsignedShort      = 34,
    /// XSD_UNSIGNED_BYTE
    UnsignedByte       = 35,
}

impl Default for DataType {
    /// Choosing boolean here as the default type because the default
    /// for `LexicalValueUnion` is a boolean false.
    fn default() -> Self { DataType::Boolean }
}

impl DataType {
    pub fn from_datatype_id(data_type_id: u8) -> Result<DataType, RDFStoreError> {
        DataType::try_from(data_type_id).map_err(|_err| UnknownDataType { data_type_id })
    }

    pub fn from_xsd_iri(iri: &str) -> Result<Self, RDFStoreError> {
        if let Some(data_type) = DATA_TYPE_MAP.get(iri) {
            Ok(data_type.clone())
        } else {
            Err(RDFStoreError::UnknownXsdDataType { data_type_iri: iri.to_string() })
        }
    }

    pub fn as_xsd_iri_str(&self) -> &'static str {
        DATA_TYPE_MAP
            .entries()
            .find_map(|(key, val)| {
                if val == self {
                    Some(key)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                panic!("You've managed to create an unknown DataType instance")
            })
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        match self {
            DataType::String | DataType::PlainLiteral => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_iri(&self) -> bool {
        match self {
            DataType::AnyUri | DataType::IriReference => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_boolean(&self) -> bool {
        match self {
            DataType::Boolean => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_date(&self) -> bool {
        match self {
            DataType::Date => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_date_time(&self) -> bool {
        match self {
            DataType::DateTime => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_decimal(&self) -> bool {
        match self {
            DataType::Decimal => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_date_time_stamp(&self) -> bool {
        match self {
            DataType::DateTimeStamp => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_duration(&self) -> bool {
        match self {
            DataType::Duration => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_signed_integer(&self) -> bool {
        // IRI_TYPES
        match self {
            DataType::Int |
            DataType::Integer |
            DataType::NegativeInteger |
            DataType::NonPositiveInteger |
            DataType::Long |
            DataType::Short => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_unsigned_integer(&self) -> bool {
        // IRI_TYPES
        match self {
            DataType::PositiveInteger |
            DataType::NonNegativeInteger |
            DataType::UnsignedByte |
            DataType::UnsignedInt |
            DataType::UnsignedShort |
            DataType::UnsignedLong => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_blank_node(&self) -> bool {
        // BLANK_NODE_TYPES
        match self {
            DataType::BlankNode => true,
            _ => false,
        }
    }
}
