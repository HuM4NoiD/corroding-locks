// The idea is to help generate enums for the different node types 
// that represent the AST for the language.
macro_rules! generate_ast {
    (
        $(
            $node:ty {$(
                $nodetype:ident {$(
                    $attr:ident: $attrtype:ty
                ),*}
            ),*}
        ),*
    ) => {
        $(
            // let $node is Expr
            // let $nodetype is Literal, Binary

            $(paste! {
                // pub struct LiteralExpr { attrs... }
                pub struct [<$nodetype $node>] {
                    $(pub $attr: $attrtype),*
                }

                // impl LiteralExpr { pub fn new(attrs...) -> Self { Self { attrs... } } }
                impl [<$nodetype $node>] {
                    pub fn new($($attr: $attrtype),*) -> Self {
                        Self {$($attr),*}
                    }
                }
            })*

            paste! {
                // pub enum Expr { Literal(LiteralExpr), Binary(BinaryExpr) }
                pub enum $node {
                    $($nodetype ([<$nodetype $node>])),*
                }
            }

            paste! {
                // pub trait ExprVisitor {
                //     fn visit_literal_expr(&self, expr: &LiteralExpr);
                //     fn visit_binary_expr(&self, expr: BinaryExpr);
                // }
                pub trait [<$node Visitor>]<R> {
                    $(fn [<visit_ $nodetype:snake _ $node:snake>](&self, [<$node:snake>]: &[<$nodetype $node>]) -> R;)*
                }

                pub trait [<Mut $node Visitor>]<R> {
                    $(fn [<visit_ $nodetype:snake _ $node:snake>](&mut self, [<$node:snake>]: &[<$nodetype $node>]) -> R;)*
                }
            }

            paste! {
                // impl Expr {
                //     pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> R {
                //         match self {
                //             Expr::Binary(expr) => visitor.visit_binary_expr(expr),
                //             Expr::Literal(expr) => visitor.visit_literal_expr(expr),
                //         }
                //     }
                // }
                impl $node {
                    pub fn accept<R>(&self, visitor: &dyn [<$node Visitor>]<R>) -> R {
                        match self {$(
                            $node::$nodetype (expr) => visitor.[<visit_ $nodetype:snake _ $node:lower>](expr)
                        ),*}
                    }

                    pub fn accept_mut<R>(&self, visitor: &mut dyn [<Mut $node Visitor>]<R>) -> R {
                        match self {$(
                            $node::$nodetype (expr) => visitor.[<visit_ $nodetype:snake _ $node:snake>](expr)
                        ),*}
                    }

                    $(
                        pub fn [<$nodetype:snake>] ($($attr: $attrtype),*) -> $node {
                            $node :: $nodetype ([<$nodetype $node>]::new($($attr),*))
                        }
                    )*
                }
            }
        )*
    };
}
