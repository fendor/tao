#![feature(option_zip, trait_alias)]

pub mod token;
pub mod span;
pub mod src;
pub mod ast;
pub mod node;
pub mod parse;

pub use crate::{
    span::Span,
    node::{Node, SrcNode},
    src::SrcId,
    token::{Token, Op, Delimiter},
};
