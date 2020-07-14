//! The main crate for RSLint, an extremely fast and customizeable linter for JavaScript powered by [rslint-parse](rslint_parse).  
//! 
//! Focused on speed, ease of use, and customizability 
//! # Speed 
//! RSLint employs various tactics for making the linting process as fast as possible, these include: 
//!  - Using a custom fast parser which retains whitespace 
//!  - Using a lookup table and trie based lexer for parsing 
//!  - Using separate distinct threads for splitting up IO bound tasks such as loading files 
//!  - Linting each file in parallel 
//!  - Running each rule from every group in parallel over the concrete syntax tree 
//! 
//! # Distinct rule types (Planned) 
//! To avoid placing constraints on the productions which can be checked, distinct types of rules are used.  
//! The main type being a [`CstRule`](rules::CstRule), a CstRule is concerned with the concrete syntax tree of a single file (or chunk of code from another file, e.g. md files).  
//! CstRules have to abide by certain rules: 
//!  - They must be [`Send`](std::marker::Send) and [`Sync`](std::marker::Sync) as the linting process is highly parallelized 
//!  - They cannot rely on the results from separate rules or files (this is impossible as linting is concurrent over files and rules) 
//!  - They may not modify files as this may cause corruption of the data if two rules attempt to do that at the same time  
//!       Rules can apply fixes to files but this has to use a special fixer interface to make sure the fixes are applied serially 
//! 
//! Some rules like import rules may have to construct a source map and/or rely on the concrete syntax tree of all of the files.  
//! To allow this functionality while keeping a sane implementation, RSLint provides a second rule type, LateRules.  
//! LateRules are concerned with all of the concrete syntax trees that have been produced, as well as all of the files that have been loaded 
//! LateRules must also abide by certain rules: 
//!  - They must be [`Send`](std::marker::Send) and [`Sync`](std::marker::Sync) just like CstRules 
//!  - They cannot rely on the result of other LateRules or any CstRules (this may be changed in the future for CstRules) 
//!  - They cannot change the files for the same reason as CstRules, they can however use a fixer interface 

#![feature(test)]

pub mod linter;
pub mod rules;
pub mod visit;
pub mod diagnostic;
pub mod formatters;
pub mod runner;
pub mod test_util;
