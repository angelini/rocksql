use std::collections::HashMap;

pub type TableId = usize;
pub type ColumnId = usize;
pub type RowId = usize;
pub type SubsetId = usize;

#[derive(Clone, Copy)]
pub enum Type {
    Bool,
    Float,
    Int,
    String,
}

#[derive(Clone)]
pub struct Column {
    pub id: ColumnId,
    pub type_: Type,
    pub sorted: bool,
}

impl Column {
    pub fn key_column(id: ColumnId) -> Column {
        Column {
            id: id,
            type_: Type::Int,
            sorted: true,
        }
    }
}

pub type Schema = Vec<(String, Type)>;

#[derive(Clone, Debug)]
pub struct Table {
    pub id: TableId,
    pub columns: Vec<(String, ColumnId)>,
}

impl Table {
    pub fn column_id(&self, name: &str) -> Option<ColumnId> {
        self.columns
            .iter()
            .find(|(col_name, _)| col_name == name)
            .map(|(_, id)| *id)
    }
}

pub struct Value<'a> {
    row_id: RowId,
    datum: &'a [u8],
}

pub struct Row {
    id: RowId,
    data: Vec<Vec<u8>>,
}

pub struct Rows {
    schema: Schema,
    rows: Vec<Row>,
}

#[derive(Clone)]
pub enum Subset {
    Ids(Vec<RowId>),
    All,
}

#[derive(Clone)]
pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
}

#[derive(Clone)]
pub enum Aggregator {
    Average,
    Count,
    Max,
    Min,
    Sum,
}
