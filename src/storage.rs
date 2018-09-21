use base::*;
use predicate::{Comparator, Predicate};

pub trait SortedStore {
    fn scan(&self, ColumnId, Predicate) -> Vec<RowId>;
    fn scan_values<'a>(&self, ColumnId, Predicate) -> Vec<&'a [u8]>;
    fn fetch<'a, 'b>(&self, ColumnId, &'a [RowId]) -> Vec<&'b [u8]>;
    fn sort(&self, ColumnId) -> Vec<RowId>;
    fn join(&self, ColumnId, ColumnId, Comparator, JoinType) -> Vec<(RowId, RowId)>;
    fn aggregate<'a>(&self, ColumnId, Aggregator) -> Vec<&'a [u8]>;
}
