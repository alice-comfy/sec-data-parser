pub use crate::document_body::*;
use crate::document_tree::parse_doc;
use crate::document_tree::DocumentTree;
use crate::error::Result;
pub use crate::schema::*;
use crate::tag::ContainerTag;
use crate::tokens::tokenize_submission;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;
use std::str::from_utf8;

mod document_body;
mod document_tree;
mod error;
mod schema;
mod tag;
mod tokens;
mod types;

pub fn parse_submission(path: &Path) -> Result<Submission> {
    let st = read_to_string(path).unwrap();
    let mut tokens = VecDeque::from(tokenize_submission(st)?);

    if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) = parse_doc(&mut tokens)
    {
        Submission::from_parts(&parts)
    } else {
        panic!("here1");
    }
}
//function for processing bytes. This is usefulf or our internal usecases, as we unpack and parse filings without permanently saving the NC files to save storage.
pub fn parse_byte_submission(bytes : &[u8]) -> Option<Submission>  {
    let st = std::str::from_utf8(bytes).unwrap().to_string();
    let mut tokens = VecDeque::from(tokenize_submission(st)?);

    if let Ok(DocumentTree::ContainerNode(ContainerTag::Submission, parts)) = parse_doc(&mut tokens)
    {
        Some(Submission::from_parts(&parts).unwrap())
    } else {
        None
    }
}
