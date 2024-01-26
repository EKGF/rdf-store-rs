// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------
#![doc(hidden)]

use {
    crate::{Graph, Namespace},
    core::str::FromStr,
    iref::iri::Iri,
    lazy_static::lazy_static,
    mime::Mime,
};

#[doc(hidden)]
pub const LOG_TARGET_CONFIG: &str = "config";
#[doc(hidden)]
pub const LOG_TARGET_SPARQL: &str = "sparql";
#[doc(hidden)]
pub const LOG_TARGET_FILES: &str = "files";
#[doc(hidden)]
pub const LOG_TARGET_DATABASE: &str = "database";

// All supported MIME types
lazy_static! {
    // As documented here: https://docs.oxfordsemantic.tech/5.6/programmatic-access-APIs.html#formats-encoding-sparql-query-results
    #[doc(hidden)]
    pub static ref TEXT_TSV: Mime = Mime::from_str("text/tab-separated-values").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_CSV: Mime = Mime::from_str("text/csv").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_X_CSV_ABBREV: Mime = Mime::from_str("text/x.csv-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_TURTLE: Mime = Mime::from_str("text/turtle").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_OWL_FUNCTIONAL: Mime = Mime::from_str("text/owl-functional").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_X_TAB_SEPARATED_VALUES_ABBREV: Mime =
        Mime::from_str("text/x.tab-separated-values-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_TRIG: Mime = Mime::from_str("application/trig").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_N_QUADS: Mime = Mime::from_str("application/n-quads").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_N_TRIPLES: Mime = Mime::from_str("application/n-triples").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_DATALOG: Mime = Mime::from_str("application/x.datalog").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_XML: Mime =
        Mime::from_str("application/sparql-results+xml").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_JSON: Mime =
        Mime::from_str("application/sparql-results+json").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_TURTLE: Mime =
        Mime::from_str("application/sparql-results+turtle").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_XML_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+xml-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_JSON_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+json-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_TURTLE_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+turtle-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_RESOURCEID: Mime =
        Mime::from_str("application/x.sparql-results+resourceid").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_NULL: Mime =
        Mime::from_str("application/x.sparql-results+null").unwrap();
}

type PrefixName<'a> = &'a str;

#[doc(hidden)]
pub const DEFAULT_BASE_IRI: &str = "https://placeholder.kg";

#[doc(hidden)]
const PREFIX_NAME_DCAT: PrefixName<'static> = "dcat:";
#[doc(hidden)]
const PREFIX_NAME_OWL: PrefixName<'static> = "owl:";
#[doc(hidden)]
const PREFIX_NAME_RDF: PrefixName<'static> = "rdf:";
#[doc(hidden)]
const PREFIX_NAME_RDFS: PrefixName<'static> = "rdfs:";
#[doc(hidden)]
const PREFIX_NAME_SKOS: PrefixName<'static> = "skos:";
#[doc(hidden)]
const PREFIX_NAME_XSD: PrefixName<'static> = "xsd:";
#[doc(hidden)]
const PREFIX_NAME_RDFOX: PrefixName<'static> = "rdfox:";

#[doc(hidden)]
const NS_STR_DCAT: &str = "http://www.w3.org/ns/dcat#";
#[doc(hidden)]
const NS_STR_OWL: &str = "http://www.w3.org/2002/07/owl#";
#[doc(hidden)]
const NS_STR_RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
#[doc(hidden)]
const NS_STR_RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
#[doc(hidden)]
const NS_STR_SKOS: &str = "http://www.w3.org/2004/02/skos/core#";
#[doc(hidden)]
const NS_STR_XSD: &str = "http://www.w3.org/2001/XMLSchema#";
#[doc(hidden)]
const NS_STR_RDFOX: &str = "http://oxfordsemantic.tech/RDFox#";

lazy_static! {
    #[doc(hidden)]
    pub static ref NS_DCAT: &'static Iri = Iri::new(NS_STR_DCAT).unwrap();
    #[doc(hidden)]
    pub static ref NS_OWL: &'static Iri = Iri::new(NS_STR_OWL).unwrap();
    #[doc(hidden)]
    pub static ref NS_RDF: &'static Iri = Iri::new(NS_STR_RDF).unwrap();
    #[doc(hidden)]
    pub static ref NS_RDFS: &'static Iri = Iri::new(NS_STR_RDFS).unwrap();
    #[doc(hidden)]
    pub static ref NS_SKOS: &'static Iri = Iri::new(NS_STR_SKOS).unwrap();
    #[doc(hidden)]
    pub static ref NS_XSD: &'static Iri = Iri::new(NS_STR_XSD).unwrap();
    #[doc(hidden)]
    pub static ref NS_RDFOX: &'static Iri = Iri::new(NS_STR_RDFOX).unwrap();
}

lazy_static! {
    #[doc(hidden)]
    pub static ref PREFIX_DCAT: Namespace = Namespace::declare(PREFIX_NAME_DCAT, NS_DCAT.deref());
    #[doc(hidden)]
    pub static ref PREFIX_OWL: Namespace = Namespace::declare(PREFIX_NAME_OWL, NS_OWL.deref());
    #[doc(hidden)]
    pub static ref PREFIX_RDF: Namespace = Namespace::declare(PREFIX_NAME_RDF, NS_RDF.deref());
    #[doc(hidden)]
    pub static ref PREFIX_RDFS: Namespace = Namespace::declare(PREFIX_NAME_RDFS, NS_RDFS.deref());
    #[doc(hidden)]
    pub static ref PREFIX_SKOS: Namespace = Namespace::declare(PREFIX_NAME_SKOS, NS_SKOS.deref());
    #[doc(hidden)]
    pub static ref PREFIX_XSD: Namespace = Namespace::declare(PREFIX_NAME_XSD, NS_XSD.deref());
    #[doc(hidden)]
    pub static ref PREFIX_RDFOX: Namespace = Namespace::declare(PREFIX_NAME_RDFOX, NS_RDFOX.deref());
}

lazy_static! {
    #[doc(hidden)]
    pub static ref DEFAULT_GRAPH_RDFOX: Graph =
        Graph::declare(PREFIX_RDFOX.deref().clone(), "DefaultTriples");
}
