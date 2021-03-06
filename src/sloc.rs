use crate::checker::Checker;
use fxhash::FxHashSet;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::*;

#[derive(Debug, Default)]
pub struct Stats {
    start: usize,
    end: usize,
    lines: FxHashSet<usize>,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("loc", 2)?;
        st.serialize_field("sloc", &self.sloc())?;
        st.serialize_field("lloc", &self.lloc())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sloc {}, lloc: {}", self.sloc(), self.lloc())
    }
}

impl Stats {
    pub fn merge(&mut self, other: &Stats) {
        for l in other.lines.iter() {
            self.lines.insert(*l);
        }
    }

    #[inline(always)]
    pub fn sloc(&self) -> f64 {
        (self.end - self.start) as f64 + 1.
    }

    #[inline(always)]
    pub fn lloc(&self) -> f64 {
        self.lines.len() as f64
    }
}

pub trait SourceLoc
where
    Self: Checker,
{
    fn compute(_node: &Node, _code: &[u8], _stats: &mut Stats, _is_func_space: bool) {}
}

#[inline(always)]
fn init(node: &Node, stats: &mut Stats, is_func_space: bool) -> usize {
    let start = node.start_position().row;

    if is_func_space {
        stats.start = start;

        let end = node.end_position().row;
        stats.end = end;
    }
    start
}

impl SourceLoc for PythonCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Python::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | String | DQUOTE | DQUOTE2 | ExpressionStatement | Block => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for MozjsCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Mozjs::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | String | DQUOTE | ExpressionStatement | StatementBlock => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for JavascriptCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Javascript::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | String | DQUOTE | ExpressionStatement | StatementBlock => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for TypescriptCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Typescript::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | String | DQUOTE | ExpressionStatement | StatementBlock => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for TsxCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Tsx::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | String | DQUOTE | ExpressionStatement | StatementBlock => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for RustCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Rust::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            LineComment | BlockComment | StringLiteral | RawStringLiteral | ExpressionStatement
            | Block => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for CppCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool) {
        use Cpp::*;

        let start = init(node, stats, is_func_space);

        match node.kind_id().into() {
            Comment | RawStringLiteral | StringLiteral | ExpressionStatement
            | CompoundStatement | LabeledStatement | DeclarationList | FieldDeclarationList => {}
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl SourceLoc for PreprocCode {}
impl SourceLoc for CcommentCode {}
impl SourceLoc for CSharpCode {}
impl SourceLoc for JavaCode {}
impl SourceLoc for GoCode {}
impl SourceLoc for CssCode {}
impl SourceLoc for HtmlCode {}
