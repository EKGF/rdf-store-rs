// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use {
    core::str::FromStr,
    iref::Iri,
    lazy_static::lazy_static,
    mime::Mime,
    crate::{Graph, Prefix},
};

pub const LOG_TARGET_CONFIG: &str = "config";
pub const LOG_TARGET_SPARQL: &str = "sparql";
pub const LOG_TARGET_FILES: &str = "files";
pub const LOG_TARGET_DATABASE: &str = "database";

// All supported MIME types
lazy_static! {
    // As documented here: https://docs.oxfordsemantic.tech/5.6/programmatic-access-APIs.html#formats-encoding-sparql-query-results
    pub static ref TEXT_TSV: Mime = Mime::from_str("text/tab-separated-values").unwrap();
    pub static ref TEXT_CSV: Mime = Mime::from_str("text/csv").unwrap();
    pub static ref TEXT_X_CSV_ABBREV: Mime = Mime::from_str("text/x.csv-abbrev").unwrap();
    pub static ref TEXT_TURTLE: Mime = Mime::from_str("text/turtle").unwrap();
    pub static ref TEXT_OWL_FUNCTIONAL: Mime = Mime::from_str("text/owl-functional").unwrap();
    pub static ref TEXT_X_TAB_SEPARATED_VALUES_ABBREV: Mime =
        Mime::from_str("text/x.tab-separated-values-abbrev").unwrap();
    pub static ref APPLICATION_TRIG: Mime = Mime::from_str("application/trig").unwrap();
    pub static ref APPLICATION_N_QUADS: Mime = Mime::from_str("application/n-quads").unwrap();
    pub static ref APPLICATION_N_TRIPLES: Mime = Mime::from_str("application/n-triples").unwrap();
    pub static ref APPLICATION_X_DATALOG: Mime = Mime::from_str("application/x.datalog").unwrap();
    pub static ref APPLICATION_SPARQL_RESULTS_XML: Mime =
        Mime::from_str("application/sparql-results+xml").unwrap();
    pub static ref APPLICATION_SPARQL_RESULTS_JSON: Mime =
        Mime::from_str("application/sparql-results+json").unwrap();
    pub static ref APPLICATION_SPARQL_RESULTS_TURTLE: Mime =
        Mime::from_str("application/sparql-results+turtle").unwrap();
    pub static ref APPLICATION_X_SPARQL_RESULTS_XML_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+xml-abbrev").unwrap();
    pub static ref APPLICATION_X_SPARQL_RESULTS_JSON_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+json-abbrev").unwrap();
    pub static ref APPLICATION_X_SPARQL_RESULTS_TURTLE_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+turtle-abbrev").unwrap();
    pub static ref APPLICATION_X_SPARQL_RESULTS_RESOURCEID: Mime =
        Mime::from_str("application/x.sparql-results+resourceid").unwrap();
    pub static ref APPLICATION_X_SPARQL_RESULTS_NULL: Mime =
        Mime::from_str("application/x.sparql-results+null").unwrap();
}

type PrefixName<'a> = &'a str;

pub const DEFAULT_BASE_IRI: &str = "https://placeholder.kg";

const PREFIX_NAME_DCAT: PrefixName<'static> = "dcat:";
const PREFIX_NAME_OWL: PrefixName<'static> = "owl:";
const PREFIX_NAME_RDF: PrefixName<'static> = "rdf:";
const PREFIX_NAME_RDFS: PrefixName<'static> = "rdfs:";
const PREFIX_NAME_SKOS: PrefixName<'static> = "skos:";
const PREFIX_NAME_XSD: PrefixName<'static> = "xsd:";
const PREFIX_NAME_RDFOX: PrefixName<'static> = "rdfox:";

const NS_STR_DCAT: &str = "http://www.w3.org/ns/dcat#";
const NS_STR_OWL: &str = "http://www.w3.org/2002/07/owl#";
const NS_STR_RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const NS_STR_RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
const NS_STR_SKOS: &str = "http://www.w3.org/2004/02/skos/core#";
const NS_STR_XSD: &str = "http://www.w3.org/2001/XMLSchema#";
const NS_STR_RDFOX: &str = "http://oxfordsemantic.tech/RDFox#";

lazy_static! {
    pub static ref NS_DCAT: Iri<'static> = Iri::new(NS_STR_DCAT).unwrap();
    pub static ref NS_OWL: Iri<'static> = Iri::new(NS_STR_OWL).unwrap();
    pub static ref NS_RDF: Iri<'static> = Iri::new(NS_STR_RDF).unwrap();
    pub static ref NS_RDFS: Iri<'static> = Iri::new(NS_STR_RDFS).unwrap();
    pub static ref NS_SKOS: Iri<'static> = Iri::new(NS_STR_SKOS).unwrap();
    pub static ref NS_XSD: Iri<'static> = Iri::new(NS_STR_XSD).unwrap();
    pub static ref NS_RDFOX: Iri<'static> = Iri::new(NS_STR_RDFOX).unwrap();
}

lazy_static! {
    pub static ref PREFIX_DCAT: Prefix = Prefix::declare(PREFIX_NAME_DCAT, *NS_DCAT.deref());
    pub static ref PREFIX_OWL: Prefix = Prefix::declare(PREFIX_NAME_OWL, *NS_OWL.deref());
    pub static ref PREFIX_RDF: Prefix = Prefix::declare(PREFIX_NAME_RDF, *NS_RDF.deref());
    pub static ref PREFIX_RDFS: Prefix = Prefix::declare(PREFIX_NAME_RDFS, *NS_RDFS.deref());
    pub static ref PREFIX_SKOS: Prefix = Prefix::declare(PREFIX_NAME_SKOS, *NS_SKOS.deref());
    pub static ref PREFIX_XSD: Prefix = Prefix::declare(PREFIX_NAME_XSD, *NS_XSD.deref());
    pub static ref PREFIX_RDFOX: Prefix = Prefix::declare(PREFIX_NAME_RDFOX, *NS_RDFOX.deref());
}

lazy_static! {
    pub static ref DEFAULT_GRAPH_RDFOX: Graph =
        Graph::declare(PREFIX_RDFOX.deref().clone(), "DefaultTriples");
}
