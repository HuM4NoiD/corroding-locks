use crate::{
    token::{Token, TokenType::*},
    value::Value,
};

generate_ast! {
    Expr {
        Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
        Grouping { expr: Box<Expr> },
        Unary { operator: Token, expr: Box<Expr> },
        Literal { value: Option<Value> }
    },

    Stmt {
        VarDec { name: Token, initializer: Option<Box<Expr>> }
    }
}

// pub enum Expr {
//     Binary {
//         left: Box<Expr>,
//         operator: Token,
//         right: Box<Expr>,
//     },
//     Grouping {
//         expr: Box<Expr>,
//     },
//     Unary {
//         operator: Token,
//         expr: Box<Expr>,
//     },
//     Literal {
//         value: Option<Value>,
//     },
// }

// pub trait Visitor<R> {
//     fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
//     fn visit_grouping_expr(&mut self, expr: &Expr) -> R;
//     fn visit_literal_expr(&mut self, literal: &Option<Value>) -> R;
//     fn visit_unary_expr(&mut self, operator: &Token, expr: &Expr) -> R;
// }

// impl Expr {
//     pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
//         match self {
//             Expr::Binary {
//                 left,
//                 operator,
//                 right,
//             } => visitor.visit_binary_expr(left, operator, right),
//             Expr::Grouping { expr } => visitor.visit_grouping_expr(expr),
//             Expr::Unary { operator, expr } => visitor.visit_unary_expr(operator, expr),
//             Expr::Literal { value } => visitor.visit_literal_expr(value),
//         }
//     }
// }
