#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

#[polars_expr(output_type=String)]
fn pig_latinnify(inputs: &[Series]) -> PolarsResult<Series> {
    let ca: &StringChunked = inputs[0].str()?;
    let out: StringChunked = ca.apply_to_buffer(|value: &str, output: &mut String| {
        if let Some(first_char) = value.chars().next() {
            write!(output, "{}{}ay", &value[1..], first_char).unwrap()
        }
    });
    Ok(out.into_series())
}

use rdkit::ROMol;

#[polars_expr(output_type=String)]
fn canonicalize(inputs: &[Series]) -> PolarsResult<Series> {
    let ca: &StringChunked = inputs[0].str()?;
    // let ca: &str = inputs[0].str()?;
    let romol = ROMol::from_smiles(ca).unwrap();
    // let romol = Molecule::new_from_smiles(ca).unwrap();
    let out: StringChunked = ca.apply_to_buffer(|value: &str, output: &mut String| {
        write!(output, "{}", romol.as_smiles()).unwrap()
    });
    Ok(out.into_series())
}
