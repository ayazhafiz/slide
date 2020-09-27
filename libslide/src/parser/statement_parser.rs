use super::{errors::*, str2rat, Parser};
use crate::diagnostics::{Diagnostic, DiagnosticRecord};
use crate::grammar::*;
use crate::scanner::types::{Token, TokenType};
use crate::utils::{PeekIter, StringUtils};
use crate::{ProgramContext, Span};

/// Parses a tokenized slide program, emitting the result and any diagnostics.
pub fn parse(
    input: Vec<Token>,
    program: &str,
    program_context: &ProgramContext,
) -> (StmtList, Vec<Diagnostic>) {
    let mut parser = ExpressionParser::new(input, program, program_context);
    (parser.parse(), parser.diagnostics)
}

pub struct ExpressionParser<'a> {
    _input: PeekIter<Token>,
    program: &'a str,
    diagnostics: Vec<Diagnostic>,
    context: &'a ProgramContext,
}

impl<'a> ExpressionParser<'a> {
    fn new(input: Vec<Token>, program: &'a str, context: &'a ProgramContext) -> Self {
        Self {
            _input: PeekIter::new(input.into_iter()),
            program,
            diagnostics: vec![],
            context,
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        let mut next_2 = self.input().peek_map_n(2, |tok| (tok.ty.clone(), tok.span));
        match (next_2.pop_front(), next_2.pop_front()) {
            (
                Some((TokenType::Variable(name), _)),
                Some((asgn_ty @ TokenType::Equal, asgn_span)),
            )
            | (
                Some((TokenType::Variable(name), _)),
                Some((asgn_ty @ TokenType::AssignDefine, asgn_span)),
            ) => {
                let Span { lo, .. } = self.input().next().unwrap().span;
                self.input().next();

                let asgn_op = if matches!(asgn_ty, TokenType::Equal) {
                    AssignmentOp::Equal(asgn_span)
                } else {
                    AssignmentOp::AssignDefine(asgn_span)
                };
                let rhs = self.expr();
                let span = (lo..rhs.span.hi).into();
                Stmt::Assignment(Assignment {
                    var: intern_str!(name),
                    asgn_op,
                    rhs,
                    span,
                })
            }

            _ => Stmt::Expr(self.expr()),
        }
    }

    fn parse_pattern(&mut self, name: String, span: Span) -> RcExpr {
        self.push_diag(IllegalPattern!(span, name));
        rc_expr!(Expr::Var(intern_str!(name)), span)
    }
}

impl<'a> Parser<StmtList> for ExpressionParser<'a> {
    type Expr = RcExpr;

    fn input(&mut self) -> &mut PeekIter<Token> {
        &mut self._input
    }

    fn push_diag(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn parse(&mut self) -> StmtList {
        let mut stmts = Vec::new();
        while !self.done() {
            stmts.push(self.parse_stmt());

            if !self.done() && !self.has_stmt_break() {
                self.extra_tokens_diag(|diag, first_tok_span| {
                    diag.with_spanned_help(
                        first_tok_span,
                        "if you meant to specify another statement, add a newline before this token"
                    )
                });
                break;
            }
        }
        StmtList::new(stmts)
    }

    fn parse_num(&mut self, num: String, span: Span) -> Self::Expr {
        let num = str2rat(&num, self.context.prec);
        rc_expr!(Expr::Const(num), span)
    }

    fn parse_variable(&mut self, name: String, span: Span) -> Self::Expr {
        rc_expr!(Expr::Var(intern_str!(name)), span)
    }

    fn parse_var_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        self.parse_pattern(name, span)
    }

    fn parse_const_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        self.parse_pattern(name, span)
    }

    fn parse_any_pattern(&mut self, name: String, span: Span) -> Self::Expr {
        self.parse_pattern(name, span)
    }

    /// Do we have another statement (on a newline)?
    fn has_stmt_break(&mut self) -> bool {
        self.peek()
            .full_span
            .clone()
            .over(self.program)
            .contains('\n')
    }
}

#[cfg(test)]
mod tests {
    parser_tests! {
        expr

        variable:                "a"
        variable_in_op_left:     "a + 1"
        variable_in_op_right:    "1 + a"
        assignment_op:           "a = 5"
        assignment_op_expr:      "a = 5 + 2 ^ 3"
    }
}
