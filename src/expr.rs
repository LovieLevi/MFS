use std::fmt::{self, Display, Formatter};
use colored::Colorize;

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Op {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "^" => Self::Pow,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Add => "+".to_string(),
            Self::Sub => "-".to_string(),
            Self::Mul => "*".to_string(),
            Self::Div => "/".to_string(),
            Self::Pow => "^".to_string(),
        }
    }
}

pub enum ExprHand {
    Var(String),
    Num(f64),
    Exp(Box<Expr>),
}

impl ExprHand {
    pub fn from_str(s: &str) -> Self {
        println!("{}", s);
        if let Ok(n) = s.parse::<f64>() {
            Self::Num(n)
        } else {
            Self::Var(s.to_string())
        }
    }

    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Self::Var(_) => Err("Cannot evaluate variable".into()),
            Self::Num(n) => Ok(*n),
            Self::Exp(e) => match e.eval() {
                Ok(Self::Num(n)) => Ok(n),
                _ => Err("Cannot evaluate expression".into()),
            },
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Var(s) => s.clone().red().bold().to_string(),
            Self::Num(n) => n.to_string().yellow().bold().to_string(),
            Self::Exp(e) => e.to_string(),
        }
    }
}

pub struct Expr {
    left: ExprHand,
    right: ExprHand,
    op: Op,
}

impl Expr {
    pub fn new(left: ExprHand, right: ExprHand, op: Op) -> Self {
        Self {
            left,
            right,
            op,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut s = s.replace(" ", "").trim().to_string();
        let mut left = None;
        let mut right = None;
        let mut op = None;

        if s.starts_with("(") && s.ends_with(")") {
            s = (&s[1..s.len() - 1]).to_string();
        }

        for (i, c) in s.chars().enumerate() {
            match c {
                '+' | '-' | '*' | '/' | '^' => {
                    left = Some(ExprHand::from_str(&s[..i]));
                    right = Some(ExprHand::from_str(&s[i + 1..]));
                    op = Some(Op::from_str(&c.to_string()));
                    break;
                }
                _ => continue,
            }
        }

        if left.is_none() || right.is_none() || op.is_none() {
            return Err("Invalid expression".into());
        }

        Ok(Self::new(left.unwrap(), right.unwrap(), op.unwrap()))
    }

    pub fn eval(&self) -> Result<ExprHand, String> {
        let left = match self.left {
            ExprHand::Exp(ref e) => match e.eval() {
                Ok(e) => e,
                Err(e) => return Err(e),
            },
            ExprHand::Num(n) => ExprHand::Num(n),
            _ => return Err("Cannot evaluate variable".into()),
        };
        let right = match self.right {
            ExprHand::Exp(ref e) => match e.eval() {
                Ok(e) => e,
                Err(e) => return Err(e),
            },
            ExprHand::Num(n) => ExprHand::Num(n),
            _ => return Err("Cannot evaluate variable".into()),
        };

        match (left, right) {
            (ExprHand::Num(l), ExprHand::Num(r)) => {
                let result = match self.op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                    Op::Pow => l.powf(r),
                };
                Ok(ExprHand::Num(result))
            }
            _ => Err("Cannot evaluate expression".into()),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{} {} {}{}", "(".bright_black(), self.left.display(), self.op.display(), self.right.display(), ")".bright_black())
    }
}
