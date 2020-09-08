use super::{extra_tokens_diag, Parser};
use crate::common::Span;
use crate::diagnostics::Diagnostic;
use crate::grammar::*;
use crate::scanner::types::Token;
use crate::utils::PeekIter;

/// Parses a tokenized slide expression pattern, emitting the result and any diagnostics.
pub fn parse(input: Vec<Token>) -> (InternedExprPat, Vec<Diagnostic>) {
    let mut parser = ExpressionPatternParser::new(input);
    (parser.parse(), parser.diagnostics)
}

pub struct ExpressionPatternParser {
    _input: PeekIter<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl Parser<InternedExprPat> for ExpressionPatternParser {
    type Expr = InternedExprPat;

    fn new(input: Vec<Token>) -> Self {
        Self {
            _input: PeekIter::new(input.into_iter()),
            diagnostics: vec![],
        }
    }

    fn input(&mut self) -> &mut PeekIter<Token> {
        &mut self._input
    }

    fn push_diag(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn parse(&mut self) -> InternedExprPat {
        let parsed = self.expr();
        if !self.done() {
            let extra_tokens_diag = extra_tokens_diag(self.input());
            self.push_diag(extra_tokens_diag);
        }
        parsed
    }

    fn parse_float(&mut self, f: f64, span: Span) -> Self::Expr {
        intern_expr_pat!(ExprPat::Const(f), span)
    }

    fn parse_variable(&mut self, name: String, span: Span) -> Self::Expr {
        self.push_diag(
            Diagnostic::span_err(
                span,
                "Variables cannot be used in an expression pattern",
                Some("unexpected variable".into()),
            )
            .with_help(format!(
                r##"consider using "${name}", "#{name}", or "_{name}" as a pattern"##,
                name = name,
            )),
        );
        intern_expr_pat!(ExprPat::VarPat(name), span)
    }

    fn parse_var_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        intern_expr_pat!(ExprPat::VarPat(name), span)
    }

    fn parse_const_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        intern_expr_pat!(ExprPat::ConstPat(name), span)
    }

    fn parse_any_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        intern_expr_pat!(ExprPat::AnyPat(name), span)
    }
}

#[cfg(test)]
mod tests {
    parser_tests! {
        expr_pat

        pattern:                 "$a"
        pattern_in_op_left:      "$a + 1"
        pattern_in_op_right:     "1 + $a"
    }
}
