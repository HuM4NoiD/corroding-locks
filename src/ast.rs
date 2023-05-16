use crate::{
    token::{Token, TokenType::*},
    value::Value,
};

// This should preferably generate:
//   + Enums for a Node type
//   + Enum Variants + Corresponding structs as Enum Variants are not considered types themselves at the time of writing
//   + Immutable and Mutable Visitors
//   + Enum impls to accept visitors and shorthands for variant construction
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
