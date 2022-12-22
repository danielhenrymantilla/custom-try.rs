use super::*;

pub
struct Visitor {
    pub macro_name: Path,
}

impl visit_mut::VisitMut for Visitor {
    fn visit_expr_mut(
        self: &'_ mut Visitor,
        expr: &'_ mut Expr,
    )
    {
        // 1. Sub-recurse
        visit_mut::visit_expr_mut(self, expr);
        // 2. Handle the `expr_try` case.
        *expr = match mem::replace(expr, Expr::Verbatim(<_>::default())) {
            | Expr::Try(ExprTry { attrs, expr: scrutinee, question_token }) => {
                let Self { macro_name } = self;
                Expr::Macro(ExprMacro {
                    attrs,
                    mac: parse_quote_spanned!(question_token.span()=>
                        #macro_name ! ( #scrutinee )
                    ),
                })
            },
            | expr => expr,
        };
    }
}
