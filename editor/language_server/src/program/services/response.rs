use libslide::Span;
use tower_lsp::lsp_types::*;

#[derive(Debug, Clone)]
pub struct ProgramLocation {
    pub uri: Url,
    pub span: Span,
}

pub struct ProgramLocationLink {
    pub origin_selection_span: Span,
    pub target_uri: Url,
    pub target_span: Span,
    pub target_selection_span: Span,
}

pub struct ProgramHoverResponse {
    pub contents: HoverContents,
    pub span: Span,
}

pub struct ProgramHighlight {
    pub kind: DocumentHighlightKind,
    pub span: libslide::Span,
}

pub enum ProgramDefinitionResponse {
    Array(Vec<ProgramLocation>),
    Link(Vec<ProgramLocationLink>),
}

#[derive(Debug, Clone)]
pub struct ProgramDiagnosticRelatedInformation {
    pub location: ProgramLocation,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ProgramDiagnostic {
    pub span: Span,
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub source: String,
    pub message: String,
    pub related_information: Vec<ProgramDiagnosticRelatedInformation>,
}