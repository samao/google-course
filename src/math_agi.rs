use tracing::info;

pub fn math_run() {
    info!("math mode run");
}

pub(crate) fn math_agi_run() {
    info!("math agi mode run")
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expression {
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(v) => Ok(v),
        Expression::Op {
            op: Operation::Add,
            left,
            right,
        } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            Ok(left + right)
        }
        Expression::Op {
            op: Operation::Multiply,
            left,
            right,
        } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            Ok(left * right)
        }
        Expression::Op {
            op: Operation::Subtract,
            left,
            right,
        } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            Ok(left - right)
        }
        Expression::Op {
            op: Operation::Divide,
            left,
            right,
        } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            if right == 0 {
                return Err("division by zero".to_string());
            }
            Ok(left / right)
        }
    }
}

#[test]
fn test_value() {
    let e = Expression::Value(42);
    assert_eq!(eval(e), Ok(42));
}

#[test]
fn test_add() {
    let e = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(1)),
        right: Box::new(Expression::Value(2)),
    };
    assert_eq!(eval(e), Ok(3));
}

#[test]
fn test_recursive() {
    let term1 = Expression::Op {
        op: Operation::Multiply,
        left: Box::new(Expression::Value(1)),
        right: Box::new(Expression::Value(2)),
    };

    let term2 = Expression::Op {
        op: Operation::Multiply,
        left: Box::new(Expression::Op {
            op: Operation::Subtract,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(3)),
    };

    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(-1)
    );
}

#[test]
fn test_error() {
    let e = Expression::Op {
        op: Operation::Divide,
        left: Box::new(Expression::Value(1)),
        right: Box::new(Expression::Value(0)),
    };
    assert_eq!(eval(e), Err("division by zero".to_string()));
}
