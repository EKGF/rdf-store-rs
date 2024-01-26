// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

extern crate alloc;

use iref::InvalidIri;
use {crate::DataType, thiserror::Error};

#[derive(Error, Debug)]
pub enum RDFStoreError {
    #[allow(dead_code)]
    #[error("Unknown Error")]
    Unknown,
    #[error("While {action}: {message}")]
    Exception { action: String, message: String },
    #[error("Unknown data type {data_type_id}")]
    UnknownDataType { data_type_id: u8 },
    #[error("Unknown value [{value}] for data type {data_type:?}")]
    UnknownValueForDataType { data_type: DataType, value: String },
    #[error("Unknown XSD data type {data_type_iri}")]
    UnknownXsdDataType { data_type_iri: String },
    #[error("Unknown literal value in N-Triples format: {value}")]
    UnknownNTriplesValue { value: String },
    #[error(
        "The multiplicity ({multiplicity}) of a cursor row exceeded the maximum number of rows \
         ({maxrow}) for query:\n{query}"
    )]
    MultiplicityExceededMaximumNumberOfRows {
        maxrow:       usize,
        multiplicity: usize,
        query:        String,
    },
    #[error("Cannot get any argument indexes from the cursor of:\n{query}")]
    CannotGetAnyArgumentIndexes { query: String },
    #[error("Maximum number of rows ({maxrow}) has been exceeded for query:\n{query}")]
    ExceededMaximumNumberOfRows { maxrow: usize, query: String },
    #[error("Could not find a license key")]
    RDFoxLicenseFileNotFound,
    #[allow(dead_code)]
    #[error("Unknown resource")]
    UnknownResourceException,
    #[error("Could not create RDFox server")]
    CouldNotCreateRDFoxServer,
    #[error("Could not connect to RDFox server")]
    CouldNotConnectToServer,
    #[error("Could not import RDF File")]
    CouldNotImportRDFFile,
    #[error("Invalid prefix name")]
    InvalidPrefixName,
    #[error("Invalid literal value")]
    InvalidLiteral,
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Represents all other cases of `ignore::Error`
    /// (see <https://docs.rs/ignore/latest/ignore/enum.Error.html>)
    #[error(transparent)]
    WalkError(#[from] ignore::Error),
    #[error(transparent)]
    InvalidIri(#[from] iref::iri::InvalidIri<String>),
    #[error("Could not parse IRI: {0:?}")]
    IriParseError(String),
    #[error(transparent)]
    IriStringParseError(#[from] iri_string::validate::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    DateParseError(#[from] chrono::ParseError),

    #[error(transparent)]
    CApiError(#[from] std::ffi::NulError),

    #[error(transparent)]
    R2D2Error(#[from] r2d2::Error),
}

#[cfg(feature = "nom-support")]
impl<I: From<&'static str>> From<RDFStoreError> for nom::Err<nom::error::Error<I>> {
    fn from(_: RDFStoreError) -> Self {
        nom::Err::Error(nom::error::Error::new(
            "unknown rdfox error".into(),
            nom::error::ErrorKind::Fail,
        ))
    }
}

impl<T: core::fmt::Debug> From<iref::IriError<T>> for RDFStoreError {
    fn from(error: iref::IriError<T>) -> Self {
        RDFStoreError::IriParseError(format!("{:?}", error))
    }
}

impl From<InvalidIri<&str>> for RDFStoreError {
    fn from(error: InvalidIri<&str>) -> Self {
        RDFStoreError::IriParseError(format!("{:?}", error))
    }
}

// impl<T> From<iref::IriError<T>> for RDFStoreError {
//     fn from(error: iref::IriError<T>) -> Self {
//         RDFStoreError::IriParseError {
//             msg: format!("{:?}", error)
//         }
//     }
// }