use std::{collections::HashMap, sync::Arc};

use kuchiki::{traits::TendrilSink, NodeRef};
use linkify::{LinkFinder, LinkKind};
use pyo3::prelude::*;
use regex::{Regex, RegexBuilder};

#[pyfunction]
fn parse_name(name: String) -> PyResult<HashMap<String, Vec<String>>> {
    let mut result = HashMap::new();

    let n = Name::parse("Jane Doe").unwrap();

    if n.is_none() {
        return Ok(result);
    }

    result.insert("given_name".to_string(), vec![n.given_name]);
    result.insert("surname".to_string(), vec![n.surname]);
    result.insert(
        "generational_suffix".to_string(),
        vec![n.generational_suffix],
    );
    result.insert("honorific_prefix".to_string(), vec![n.honorific_prefix]);
    result.insert("display_full".to_string(), vec![n.display_full]);

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn html_parsing_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_name, m)?)?;
    Ok(())
}
