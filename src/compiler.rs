use std::collections::HashMap;

use base::*;
use predicate::{BinaryCompare, Comparator, ComparisonType, Expression, Operation, Predicate};
use relation::{Projection, Selection};

#[derive(Clone)]
pub enum ColumnOperation {
    And(SubsetId, SubsetId),
    Or(SubsetId, SubsetId),
    Scan(ColumnId, Predicate),
    ScanValues(ColumnId, Predicate),
    Fetch(ColumnId, SubsetId),
    Sort(ColumnId),
    Join(ColumnId, ColumnId, Comparator, JoinType),
    Aggregate(ColumnId, Aggregator),
}

impl ColumnOperation {
    pub fn required_columns(&self) -> Vec<ColumnId> {
        match self {
            ColumnOperation::Scan(col, _) |
            ColumnOperation::ScanValues(col, _) |
            ColumnOperation::Fetch(col, _) |
            ColumnOperation::Sort(col) |
            ColumnOperation::Aggregate(col, _) => vec![*col],
            ColumnOperation::Join(left, right, _, _) => vec![*left, *right],
            _ => vec![],
        }
    }

    pub fn required_subsets(&self) -> Vec<SubsetId> {
        match self {
            ColumnOperation::And(left, right) |
            ColumnOperation::Or(left, right) => vec![*left, *right],
            ColumnOperation::Fetch(_, sub) => vec![*sub],
            _ => vec![]
        }
    }
}

type IdOperation = (ColumnId, ColumnOperation);

const ALL_ID: SubsetId = 0;

#[derive(Clone)]
pub struct Context {
    pub tables: HashMap<TableId, Table>,
    pub columns: HashMap<ColumnId, Column>,
    pub operations: HashMap<ColumnId, ColumnOperation>,
    pub subsets: HashMap<SubsetId, Subset>,
    next_id: usize,
}

impl Context {
    pub fn schema(&self, table: &Table) -> Schema {
        table.columns
            .iter()
            .map(|(name, id)| {
                (name.to_string(), *self.columns.get(id).expect("Column not found in ctx.").type_)
            })
            .collect()
    }

    fn new() -> Context {
        let mut subsets = HashMap::new();
        subsets.insert(ALL_ID, Subset::All);
        Context {
            tables: HashMap::new(),
            columns: HashMap::new(),
            operations: HashMap::new(),
            subsets: subsets,
            next_id: 1,
        }
    }

    fn gen_column_id(&mut self) -> ColumnId {
        unimplemented!()
    }
}

trait ToColumnOperations {
    fn to_operations(&self, &Context) -> Context;
}

impl ToColumnOperations for Projection {
    fn to_operations(&self, ctx: &Context) -> Context {
        let mut ctx = ctx.clone();
        let table = ctx.tables.get(&self.table).unwrap().clone();
        for name in &self.columns {
            let id = match table.column_id(&name) {
                Some(id) => id,
                None => panic!(format!("Cannot find {:?} in {:?}", name, table)),
            };
            let column = ctx.columns.get(&id).unwrap().clone();
            let new_id = ctx.gen_column_id();
            ctx.columns.insert(new_id, column);
            ctx.operations.insert(
                new_id,
                ColumnOperation::Fetch(id, ALL_ID),
            );
        }
        ctx
    }
}

fn comparison_to_operations(table: &Table, compare: &BinaryCompare) -> ColumnOperation {
    if compare.comparison_type() == ComparisonType::Constant ||
        compare.comparison_type() == ComparisonType::TwoVariables
    {
        unimplemented!()
    }
    let column_name = compare.first_symbol().unwrap();
    let column_id = table.columns.get(&column_name).unwrap();
    ColumnOperation::Scan(*column_id, Predicate::from_compare(compare))
}

fn predicate_to_operations(
    ctx: &mut Context,
    table: &Table,
    predicate: &Predicate,
) -> Vec<IdOperation> {
    let mut operations = vec![];
    let left_id = match *predicate.left {
        Expression::Predicate(ref pred) => {
            let mut ops = predicate_to_operations(ctx, table, pred);
            let id = ops.last().unwrap().0;
            operations.append(&mut ops);
            id
        }
        Expression::Compare(ref cmp) => {
            let id = ctx.gen_column_id();
            operations.push((id, comparison_to_operations(table, cmp)));
            id
        }
    };
    if let Some(right) = &predicate.right {
        let right_id = match right {
            box Expression::Predicate(ref pred) => {
                let mut ops = predicate_to_operations(ctx, table, pred);
                let id = ops.last().unwrap().0;
                operations.append(&mut ops);
                id
            }
            box Expression::Compare(ref cmp) => {
                let id = ctx.gen_column_id();
                operations.push((id, comparison_to_operations(table, cmp)));
                id
            }
        };
        let new_id = ctx.gen_column_id();
        match predicate.operation {
            Operation::And => operations.push((new_id, ColumnOperation::And(left_id, right_id))),
            Operation::Or => operations.push((new_id, ColumnOperation::Or(left_id, right_id))),
        }
    }
    operations
}

impl ToColumnOperations for Selection {
    fn to_operations(&self, ctx: &Context) -> Context {
        let mut ctx = ctx.clone();
        let table = ctx.tables.get(&self.table).unwrap().clone();

        for predicate in &self.predicates {
            for (id, op) in predicate_to_operations(&mut ctx, &table, predicate) {
                ctx.columns.insert(id, Column::key_column(id));
                ctx.operations.insert(id, op);
            }
        }
        ctx
    }
}
