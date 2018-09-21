use std::collections::HashMap;

use base::*;
use predicate::{BinaryCompare, Predicate};

struct Persistent {
    schema: Schema,
}

pub struct Projection {
    pub table: TableId,
    pub columns: Vec<String>,
}

struct Rename {
    table: TableId,
    names: HashMap<String, String>,
}

pub struct Selection {
    pub table: TableId,
    pub predicates: Vec<Predicate>,
}

struct Sort {
    table: TableId,
    columns: Vec<String>,
}

struct Join {
    left: TableId,
    right: TableId,
    compare: BinaryCompare,
    join_type: JoinType,
}

struct Aggregate {
    table: TableId,
    group: Vec<String>,
    aggregates: HashMap<String, Aggregator>,
}

enum Relation {
    Persistent(Persistent),
    Projection(Projection),
    Rename(Rename),
    Selection(Selection),
    Sort(Sort),
    Join(Join),
    Aggregate(Aggregate),
}
