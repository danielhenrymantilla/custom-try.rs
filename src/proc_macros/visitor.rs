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
        if let &mut Expr::Try(ref mut expr_try) = expr {
            let ExprTry { attrs, expr: scrutinee, question_token } =
                ::core::mem::replace(expr_try, parse_quote!(()?))
            ;
            let Self { macro_name } = self;
            *expr = Expr::Macro(ExprMacro {
                attrs,
                mac: parse_quote_spanned!(question_token.span()=>
                    #macro_name ! ( #scrutinee )
                ),
            });
        }
    }
}
