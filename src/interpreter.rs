use std::collections::HashMap;

use base::*;
use compiler::{ColumnOperation, Context};
use storage::SortedStore;

struct ColumnValues<'a> {
    type_: Type,
    values: Vec<&'a [u8]>,
}

struct ValueCache<'a> {
    values: HashMap<ColumnId, ColumnValues<'a>>,
}

impl<'a> ValueCache<'a> {
    fn new() -> ValueCache<'a> {
        ValueCache { values: HashMap::new() }
    }
}

struct Task {
    table: TableId,
    amount: usize,
}

trait Interpreter {
    fn take(&Context, &SortedStore, &Task) -> Rows;
}

type SimpleInterpreter = ();

fn eval(
    cache: &mut ValueCache,
    ctx: &Context,
    store: &SortedStore,
    op: &ColumnOperation,
    id: &ColumnId,
) {
    for required_id in &op.required_columns() {
        let required_op = ctx.operations.get(required_id).expect(
            "Dependant column operation not found in ctx",
        );
        eval(cache, ctx, store, required_op, required_id)
    }

    let values = cache.values.get(id).expect("Values not found in cache");
    match op {
        ColumnOperation::ScanValues(parent_id, predicate) => {
            cache.values.insert(
                *id,
                store.scan_values(*parent_id, *predicate),
            );
        }
    }
}

fn build_rows(cache: &ValueCache, table: &Table) -> Rows {
    unimplemented!()
}

impl Interpreter for SimpleInterpreter {
    fn take(ctx: &Context, store: &SortedStore, task: &Task) -> Rows {
        let mut cache = ValueCache::new();
        let table = ctx.tables.get(&task.table).expect("Table not found in ctx");
        for id in table.columns.iter().map(|(_, id)| id) {
            if !cache.values.contains_key(id) {
                let op = ctx.operations.get(id).expect(
                    "Column operation not found in ctx",
                );
                eval(&mut cache, ctx, store, op, id)
            }
        }
        build_rows(&cache, &table)
    }
}
