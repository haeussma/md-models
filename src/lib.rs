pub mod datamodel;
pub mod exporters;
pub mod validation;

pub(crate) mod attribute;
pub(crate) mod object;
pub(crate) mod primitives;
pub(crate) mod schema;
pub(crate) mod xmltype;

pub(crate) mod json {
    mod datatype;
    pub(crate) mod parser;
}

pub(crate) mod markdown {
    pub(crate) mod frontmatter;
    pub(crate) mod parser;
}

use exporters::Templates;
use markdown::parser::parse_markdown;
use pyo3::prelude::*;

#[pyfunction]
fn generate(schema: &str, template: Templates) -> PyResult<String> {
    let mut model = parse_markdown(schema).expect("Could not parse markdown");
    let code = exporters::render_jinja_template(&template, &mut model);

    match code {
        Ok(code) => Ok(code),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            e.to_string(),
        )),
    }
}

#[pymodule]
fn mdmodels(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_class::<Templates>()?;
    Ok(())
}
