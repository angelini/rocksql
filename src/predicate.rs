use base::*;

#[derive(Clone)]
pub enum Comparator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Clone)]
pub enum ExpressionValue {
    Constant(Type, Vec<u8>),
    Symbol(String),
}

#[derive(PartialEq)]
pub enum ComparisonType {
    Constant,
    OneVariable,
    TwoVariables,
}

#[derive(Clone)]
pub struct BinaryCompare {
    pub left: ExpressionValue,
    pub right: ExpressionValue,
    pub comparator: Comparator,
}

impl BinaryCompare {
    pub fn comparison_type(&self) -> ComparisonType {
        match (&self.left, &self.right) {
            (ExpressionValue::Constant(_, _), ExpressionValue::Constant(_, _)) => {
                ComparisonType::Constant
            }
            (ExpressionValue::Symbol(_), ExpressionValue::Symbol(_)) => {
                ComparisonType::TwoVariables
            }
            _ => ComparisonType::OneVariable,
        }
    }

    pub fn first_symbol(&self) -> Option<String> {
        match self.left {
            ExpressionValue::Symbol(ref sym) => Some(sym.clone()),
            ExpressionValue::Constant(_, _) => {
                match self.right {
                    ExpressionValue::Symbol(ref sym) => Some(sym.clone()),
                    ExpressionValue::Constant(_, _) => None,
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Predicate(Predicate),
    Compare(BinaryCompare),
}

#[derive(Clone)]
pub enum Operation {
    And,
    Or,
}

#[derive(Clone)]
pub struct Predicate {
    pub left: Box<Expression>,
    pub right: Option<Box<Expression>>,
    pub operation: Operation,
}

impl Predicate {
    pub fn from_compare(compare: &BinaryCompare) -> Predicate {
        Predicate {
            left: box Expression::Compare(compare.clone()),
            right: None,
            operation: Operation::And,
        }
    }

    pub fn one_variable_comparisons(&self) -> Vec<BinaryCompare> {
        self.comparisons()
            .into_iter()
            .filter(|cmp| cmp.comparison_type() == ComparisonType::TwoVariables)
            .collect()
    }

    pub fn two_variable_comparisons(&self) -> Vec<BinaryCompare> {
        self.comparisons()
            .into_iter()
            .filter(|cmp| cmp.comparison_type() == ComparisonType::TwoVariables)
            .collect()
    }

    fn comparisons(&self) -> Vec<BinaryCompare> {
        let mut comparisons = vec![];
        match *self.left {
            Expression::Predicate(ref pred) => comparisons.append(&mut pred.comparisons()),
            Expression::Compare(ref cmp) => comparisons.push(cmp.clone()),
        }
        if let Some(ref right) = self.right {
            match right {
                box Expression::Predicate(ref pred) => comparisons.append(&mut pred.comparisons()),
                box Expression::Compare(ref cmp) => comparisons.push(cmp.clone()),
            }
        }
        comparisons
    }
}
