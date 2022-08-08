extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::bridge::{server, TokenTree, self};
use std::collections::Bound;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::vec::IntoIter;

use proc_macro::{Delimiter, Level, LineColumn, Spacing};


//#[derive(Clone)]
//pub struct TokenStream;
type TokenStream = proc_macro2::TokenStream;

pub struct TokenStreamBuilder {
    acc: TokenStream,
}

impl TokenStreamBuilder {
    fn new() -> TokenStreamBuilder {
        TokenStreamBuilder {
            acc: TokenStream::new(),
        }
    }

    fn push(mut self, stream: TokenStream) -> Self {
        self.acc.extend(stream.into_iter());
        self
    }

    fn build(self) -> TokenStream {
        self.acc
    }
}

#[derive(Clone)]
pub struct SourceFile {
    // FIXME stub
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Span();


/// A structure representing a diagnostic message and associated children
/// messages.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
}

impl Diagnostic {
    /// Creates a new diagnostic with the given `level` and `message`.
    pub fn new<T: Into<String>>(level: Level, message: T) -> Diagnostic {
        Diagnostic { level, message: message.into(), spans: vec![], children: vec![] }
    }
}

pub struct Symbol(u32);

pub struct FreeFunctions;

#[derive(Default)]
pub struct RustAnalyzer {
}

impl server::Types for RustAnalyzer {
    type FreeFunctions = FreeFunctions;
    type TokenStream = TokenStream;
    type SourceFile = SourceFile;
    type MultiSpan = Vec<Span>;
    type Diagnostic = Diagnostic;
    type Span = Span;
    type Symbol = Symbol;
}

impl server::FreeFunctions for RustAnalyzer {
    fn track_env_var(&mut self, _var: &str, _value: Option<&str>) {
        // FIXME: track env var accesses
        // https://github.com/rust-lang/rust/pull/71858
    }
    fn track_path(&mut self, _path: &str) {}

    fn literal_from_str(
        &mut self,
        s: &str,
    ) -> Result<bridge::Literal<Self::Span, Self::Symbol>, ()> {
        unimplemented!()
    }
}

impl server::TokenStream for RustAnalyzer {
    fn is_empty(&mut self,self_: &Self::TokenStream) -> bool {
        todo!()
    }

    fn expand_expr(&mut self,self_: &Self::TokenStream) -> Result<Self::TokenStream,()> {
        todo!()
    }

    fn from_str(&mut self,src: &str) -> Self::TokenStream {
        TokenStreamBuilder::new().push(src.parse().unwrap()).build()
    }

    fn to_string(&mut self,self_: &Self::TokenStream) -> String {
        todo!()
    }

    fn from_token_tree(&mut self,tree:TokenTree<Self::TokenStream,Self::Span,Self::Symbol>) -> Self::TokenStream {
        todo!()
    }

    fn concat_trees(&mut self,base:Option<Self::TokenStream>,trees:Vec<TokenTree<Self::TokenStream,Self::Span,Self::Symbol>>) -> Self::TokenStream {
        todo!()
    }

    fn concat_streams(&mut self,base:Option<Self::TokenStream>,streams:Vec<Self::TokenStream>) -> Self::TokenStream {
        todo!()
    }

    fn into_trees(&mut self,self_:Self::TokenStream) -> Vec<TokenTree<Self::TokenStream,Self::Span,Self::Symbol>> {
        todo!()
    }
}


impl server::SourceFile for RustAnalyzer {
    // FIXME these are all stubs
    fn eq(&mut self, _file1: &Self::SourceFile, _file2: &Self::SourceFile) -> bool {
        true
    }
    fn path(&mut self, _file: &Self::SourceFile) -> String {
        String::new()
    }
    fn is_real(&mut self, _file: &Self::SourceFile) -> bool {
        true
    }
}

impl server::Diagnostic for RustAnalyzer {
    fn new(&mut self, level: Level, msg: &str, spans: Self::MultiSpan) -> Self::Diagnostic {
        let mut diag = Diagnostic::new(level, msg);
        diag.spans = spans;
        diag
    }

    fn sub(
        &mut self,
        _diag: &mut Self::Diagnostic,
        _level: Level,
        _msg: &str,
        _spans: Self::MultiSpan,
    ) {
        // FIXME handle diagnostic
        //
    }

    fn emit(&mut self, _diag: Self::Diagnostic) {
        // FIXME handle diagnostic
        // diag.emit()
    }
}

impl server::Span for RustAnalyzer {
    fn debug(&mut self,self_:Self::Span) -> String {
        todo!()
    }

    fn source_file(&mut self,self_:Self::Span) -> Self::SourceFile {
        todo!()
    }

    fn parent(&mut self,self_:Self::Span) -> Option<Self::Span> {
        todo!()
    }

    fn source(&mut self,self_:Self::Span) -> Self::Span {
        todo!()
    }

    fn start(&mut self,self_:Self::Span) -> LineColumn {
        todo!()
    }

    fn end(&mut self,self_:Self::Span) -> LineColumn {
        todo!()
    }

    fn before(&mut self,self_:Self::Span) -> Self::Span {
        todo!()
    }

    fn after(&mut self,self_:Self::Span) -> Self::Span {
        todo!()
    }

    fn join(&mut self,self_:Self::Span,other:Self::Span) -> Option<Self::Span> {
        todo!()
    }

    fn subspan(&mut self,self_:Self::Span,start:Bound<usize>,end:Bound<usize>) -> Option<Self::Span> {
        todo!()
    }

    fn resolved_at(&mut self,self_:Self::Span,at:Self::Span) -> Self::Span {
        todo!()
    }

    fn source_text(&mut self,self_:Self::Span) -> Option<String> {
        todo!()
    }

    fn save_span(&mut self,self_:Self::Span) -> usize {
        todo!()
    }

    fn recover_proc_macro_span(&mut self,id:usize) -> Self::Span {
        todo!()
    }
}

impl server::MultiSpan for RustAnalyzer {
    fn new(&mut self,) -> Self::MultiSpan {
        todo!()
    }

    fn push(&mut self,self_: &mut Self::MultiSpan,span:Self::Span) {
        todo!()
    }
}

impl server::Server for RustAnalyzer {    
    fn globals(&mut self) -> bridge::ExpnGlobals<Self::Span> {
        bridge::ExpnGlobals {
            def_site: Span(),
            call_site: Span(),
            mixed_site: Span(),
        }
    }

    fn intern_symbol(ident: &str) -> Self::Symbol {
        todo!()
    }

    fn with_symbol_string(symbol: &Self::Symbol,f:impl FnOnce(&str)) {
        todo!()
    }
}

impl server::Symbol for RustAnalyzer {
    fn normalize_and_validate_ident(&mut self,string: &str) -> Result<Self::Symbol,()> {
        todo!()
    }
}


