//! Rust compiler plugin functions.

use syntax::ast::{Expr, Ident, Path};
use syntax::ast::Expr_::{ExprField, ExprLit};
use syntax::ast::Lit_::LitInt;
use syntax::ast::LitIntType::SignedIntLit;
use syntax::ast::IntTy::TyI64;
use syntax::ast::Sign;
use syntax::codemap::{Span, Spanned, DUMMY_SP};
use syntax::parse::token::intern;
use syntax::ptr::P;

pub static NODE_ID: u32 = 4294967295;

/// Create the `ExprField` expression `expr`.`field_name`.
pub fn field_access(expr: P<Expr>, path: &Path, position: Span, field_name: String) -> P<Expr> {
    let syntax_context = path.segments[0].identifier.ctxt;
    let ident = Ident::new(intern(&field_name), syntax_context);
    P(Expr {
        id: NODE_ID,
        node: ExprField(expr, Spanned {
            node: ident,
            span: position,
        }),
        span: position,
    })
}

/// Converts a number to an `P<Expr>`.
pub fn number_literal(number: u64) -> P<Expr> {
    P(Expr {
        id: NODE_ID,
        node: ExprLit(P(Spanned {
            node: LitInt(number, SignedIntLit(TyI64, Sign::Plus)),
            span: DUMMY_SP,
        })),
        span: DUMMY_SP,
    })
}
