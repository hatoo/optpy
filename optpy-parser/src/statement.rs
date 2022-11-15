use rustpython_parser::ast::{Stmt, StmtKind};

use crate::{expression::Expr, BinaryOperator};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Statement {
    Assign(Assign),
    Expression(Expr),
    If(If),
    Func {
        name: String,
        args: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Option<Expr>),
    While(While),
    Break,
    For(For),
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct Assign {
    pub target: Expr,
    pub value: Expr,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct If {
    pub test: Expr,
    pub body: Vec<Statement>,
    pub orelse: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Func {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct While {
    pub test: Expr,
    pub body: Vec<Statement>,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]

pub struct For {
    pub(crate) target: Expr,
    pub(crate) iter: Expr,
    pub(crate) body: Vec<Statement>,
}

impl Statement {
    pub fn parse(statement: &StmtKind) -> Self {
        match statement {
            StmtKind::Assign {
                targets,
                value,
                type_comment: _,
            } => {
                assert_eq!(targets.len(), 1);
                let target = Expr::parse(&targets[0].node);
                let value = Expr::parse(&value.node);
                Self::Assign(Assign { target, value })
            }
            StmtKind::Expr { value } => Self::Expression(Expr::parse(&value.node)),
            StmtKind::If { test, body, orelse } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                let orelse = parse_statements(orelse);
                Self::If(If { test, body, orelse })
            }
            StmtKind::FunctionDef {
                decorator_list: _,
                returns: _,
                name,
                args,
                body,
                type_comment: _,
            } => {
                let name = name.to_string();
                let args = args.args.iter().map(|arg| arg.node.arg.clone()).collect();
                let body = parse_statements(body);
                Self::Func { name, args, body }
            }
            StmtKind::Return { value } => {
                let value = value.as_ref().map(|value| Expr::parse(&value.node));
                Self::Return(value)
            }
            StmtKind::While {
                test,
                body,
                orelse: _,
            } => {
                let test = Expr::parse(&test.node);
                let body = parse_statements(body);
                Self::While(While { test, body })
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse: _,
                type_comment: _,
            } => {
                let target = Expr::parse(&target.node);
                let iter = Expr::parse(&iter.node);
                let body = parse_statements(body);
                Self::For(For { target, iter, body })
            }
            StmtKind::Break => Statement::Break,
            StmtKind::AugAssign { target, op, value } => {
                let target = Expr::parse(&target.node);
                let value = Expr::parse(&value.node);
                Statement::Assign(Assign {
                    target: target.clone(),
                    value: Expr::BinaryOperation {
                        left: Box::new(target),
                        right: Box::new(value),
                        op: BinaryOperator::parse(op),
                    },
                })
            }
            statement => todo!("{:?}", statement),
        }
    }
}

fn parse_statements(statements: &[Stmt]) -> Vec<Statement> {
    statements
        .iter()
        .map(|s| Statement::parse(&s.node))
        .collect()
}
