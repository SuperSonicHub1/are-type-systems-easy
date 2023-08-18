/// Implementing https://langdev.stackexchange.com/a/2693
/// as a challenge to myself.

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Bool,
    Int,
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    BoolLiteral(bool),
    IntLiteral(i64),
    If {
        condition: Box<Expression>,
        consequent: Box<Expression>,
        alternative: Box<Expression>,
    },
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Mulitplication(Box<Expression>, Box<Expression>),
    Equality(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::IntLiteral(value)
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        Self::BoolLiteral(value)
    }
}

fn infer(expr: &Expression) -> Option<Type> {
    match expr {
        Expression::BoolLiteral(_) => Some(Type::Bool),
        Expression::IntLiteral(_) => Some(Type::Int),
        Expression::If {
            condition,
            consequent,
            alternative,
        } if infer(condition) == Some(Type::Bool) => {
            let consequent_type = infer(consequent);
            let alternative_type = infer(alternative);
            if consequent_type == alternative_type {
                consequent_type
            } else {
                None
            }
        }
        Expression::Addition(left, right)
            if infer(left) == Some(Type::Int) && infer(right) == Some(Type::Int) =>
        {
            Some(Type::Int)
        }
        Expression::Subtraction(left, right)
            if infer(left) == Some(Type::Int) && infer(right) == Some(Type::Int) =>
        {
            Some(Type::Int)
        }
        Expression::Mulitplication(left, right)
            if infer(left) == Some(Type::Int) && infer(right) == Some(Type::Int) =>
        {
            Some(Type::Int)
        }
        Expression::Equality(left, right) => {
            let left_type = infer(left);
            let right_type = infer(right);
            if left_type == right_type {
                left_type
            } else {
                None
            }
        }
        Expression::LessThan(left, right)
            if infer(left) == Some(Type::Int) && infer(right) == Some(Type::Int) =>
        {
            Some(Type::Bool)
        }
        Expression::GreaterThan(left, right)
            if infer(left) == Some(Type::Int) && infer(right) == Some(Type::Int) =>
        {
            Some(Type::Bool)
        }
        _ => None,
    }
}

fn main() {
    let expressions = vec![
        true.into(),
        128.into(),
        Expression::Addition(Box::new(128.into()), Box::new(128.into())),
        Expression::Addition(Box::new(128.into()), Box::new(false.into())),
        Expression::Subtraction(Box::new(128.into()), Box::new(128.into())),
        Expression::Subtraction(Box::new(128.into()), Box::new(false.into())),
        Expression::Mulitplication(Box::new(128.into()), Box::new(128.into())),
        Expression::Mulitplication(Box::new(128.into()), Box::new(false.into())),
        Expression::LessThan(Box::new(128.into()), Box::new(128.into())),
        Expression::LessThan(Box::new(128.into()), Box::new(false.into())),
        Expression::GreaterThan(Box::new(128.into()), Box::new(128.into())),
        Expression::GreaterThan(Box::new(128.into()), Box::new(false.into())),
        Expression::GreaterThan(Box::new(128.into()), Box::new(128.into())),
        Expression::GreaterThan(Box::new(false.into()), Box::new(false.into())),
        Expression::Equality(Box::new(128.into()), Box::new(false.into())),
        Expression::Equality(Box::new(128.into()), Box::new(128.into())),
        Expression::Equality(Box::new(false.into()), Box::new(false.into())),
        Expression::If {
            condition: Box::new(true.into()),
            consequent: Box::new(true.into()),
            alternative: Box::new(true.into()),
        },
        Expression::If {
            condition: Box::new(false.into()),
            consequent: Box::new(0.into()),
            alternative: Box::new(1.into()),
        },
        Expression::If {
            condition: Box::new(0.into()),
            consequent: Box::new(0.into()),
            alternative: Box::new(1.into()),
        },
        Expression::If {
            condition: Box::new(0.into()),
            consequent: Box::new(true.into()),
            alternative: Box::new(1.into()),
        },
    ];
    for expression in expressions {
        let inference = infer(&expression);
        println!("infer({expression:?}) = {inference:?}")
    }
}
