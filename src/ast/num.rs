use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Operator {
    Neg(Box<Num>),
    Add(Box<Num>, Box<Num>),
    Sub(Box<Num>, Box<Num>),
    Mul(Box<Num>, Box<Num>),
    Div(Box<Num>, Box<Num>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Num {
    Value(isize),
    Oper(Operator),
    PostValue(PostValue),
}
impl Num {
    pub fn pre_eval(&self) -> Self {
        match self {
            Self::Oper(op) => {
                match op {
                    Operator::Neg(child) => {
                        match child.pre_eval() {
                            Self::Value(operand) => Self::Value(-operand),
                            _ => self.clone(),
                        }
                    },
                    Operator::Add(lchild, rchild) => {
                        match (lchild.pre_eval(), rchild.pre_eval()) {
                            (Self::Value(lhs), Self::Value(rhs)) => Self::Value(lhs + rhs),
                            _ => self.clone(),
                        }
                    },
                    Operator::Sub(lchild, rchild) => {
                        match (lchild.pre_eval(), rchild.pre_eval()) {
                            (Self::Value(lhs), Self::Value(rhs)) => Self::Value(lhs - rhs),
                            _ => self.clone(),
                        }
                    },
                    Operator::Mul(lchild, rchild) => {
                        match (lchild.pre_eval(), rchild.pre_eval()) {
                            (Self::Value(lhs), Self::Value(rhs)) => Self::Value(lhs * rhs),
                            _ => self.clone(),
                        }
                    },
                    Operator::Div(lchild, rchild) => {
                        match (lchild.pre_eval(), rchild.pre_eval()) {
                            (Self::Value(lhs), Self::Value(rhs)) => Self::Value(lhs / rhs),
                            _ => self.clone(),
                        }
                    },
                }
            },
            _ => self.clone(),
        }
    }
    pub fn post_eval(&self, context: &BoardContext) -> isize {
        match self {
            Self::Value(operand) => *operand,
            Self::PostValue(operand) => operand.post_eval(context),
            Self::Oper(op) => {
                match op {
                    Operator::Neg(operand) => -operand.post_eval(context),
                    Operator::Add(lhs, rhs) => lhs.post_eval(context) + rhs.post_eval(context),
                    Operator::Sub(lhs, rhs) => lhs.post_eval(context) - rhs.post_eval(context),
                    Operator::Mul(lhs, rhs) => lhs.post_eval(context) * rhs.post_eval(context),
                    Operator::Div(lhs, rhs) => lhs.post_eval(context) / rhs.post_eval(context),
                }
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoardContext {
    width: usize,
    height: usize,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum PostValue {
    Width,
    Height,
    ArgOf {
        symbol: String,
        index: usize,
    },
}
impl PostValue {
    pub fn post_eval(&self, context: &BoardContext) -> isize {
        match self {
            Self::Width => context.width as isize,
            Self::Height => context.height as isize,
            Self::ArgOf { symbol, index } => {},
        }
    }
}