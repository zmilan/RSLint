//! A formatter which outputs colored diagnostics in a compact format to the terminal

use codespan_reporting::diagnostic::{Severity, Diagnostic};
use crate::linter::file_walker::FileWalker;
use super::Formatter;
use rayon::prelude::*;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format(&self, diagnostics: &Vec<Diagnostic<&str>>, walker: &FileWalker) {
        let sorted_files = diagnostics.clone().par_sort_by(|a, b| {
            if a.labels.first().is_some() && b.labels.first().is_some() {
                if a.labels.first().unwrap().file_id == b.labels.first().unwrap().file_id {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        });
    }
}