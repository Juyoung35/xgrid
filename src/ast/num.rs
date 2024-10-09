pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

pub enum Num {
    Value(isize),
    UnaryExpr {
        op: Operator,
        child: Box<Num>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Num>,
        rhs: Box<Num>,
    },
    PostValue(PostValue),
}
impl Num {
    pub fn pre_eval(&self, node: &Self) -> Self {
        match node {
            Self::UnaryExpr { op, child } => {
                let child = self.pre_eval(child);
                match op {
                    Operator::Neg => -child,
                    _ => child,
                }
            },
            Self::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.pre_eval(lhs);
                let rhs_ret = self.pre_eval(rhs);

                match op {
                    Operator::Add => lhs_ret + rhs_ret,
                    Operator::Sub => lhs_ret - rhs_ret,
                    Operator::Mul => lhs_ret * rhs_ret,
                    Operator::Div => lhs_ret / rhs_ret,
                }
            },
            other => other,
        }
    }
}

pub enum PostValue {
    Width,
    Height,
}