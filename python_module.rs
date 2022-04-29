use pyo3::prelude::*;
use serde_json::{Value};
use tree_sitter::{Parser, Language};
//use serde_json::json; 
use crate::syntax_checker::has_errors as syn_check;
use crate::syntax_checker::get_errors as syn_errors;
use crate::transducer::translate as man_2_ofn;


#[pyfunction]
fn syntax_check(input: &str) -> bool { 
    let mut parser = Parser::new(); 
    parser.set_language(tree_sitter_manchester::language()).expect("Error loading manchester grammar"); 
    let tree = parser.parse(input, None).unwrap();
    !syn_check(&tree)
}

#[pyfunction]
fn get_syntax_errors(input: &str) -> Vec<String> { 
    let mut parser = Parser::new(); 
    parser.set_language(tree_sitter_manchester::language()).expect("Error loading manchester grammar"); 
    let tree = parser.parse(input, None).unwrap();
    syn_errors(&tree)
}

#[pyfunction]
fn get_ofn(input: &str) -> String { 
    let mut parser = Parser::new(); 
    parser.set_language(tree_sitter_manchester::language()).expect("Error loading manchester grammar"); 
    let tree = parser.parse(input, None).unwrap();
    let ofn = man_2_ofn(&tree.root_node(), input); 
    ofn.to_string()
}




/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn man_parse_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(get_signature, m)?)?;
    m.add_function(wrap_pyfunction!(syntax_check, m)?)?;
    m.add_function(wrap_pyfunction!(get_syntax_errors, m)?)?;
    m.add_function(wrap_pyfunction!(get_ofn, m)?)?;

    Ok(())
}
