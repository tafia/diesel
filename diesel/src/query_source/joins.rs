use {QuerySource, Table};
use query_builder::*;
use expression::SelectableExpression;
use types::Nullable;

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct InnerJoinSource<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> InnerJoinSource<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        InnerJoinSource {
            left: left,
            right: right,
        }
    }
}

impl<Left, Right> QuerySource for InnerJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
{
    fn from_clause(&self, out: &mut QueryBuilder) -> BuildQueryResult {
        try!(self.left.from_clause(out));
        out.push_sql(" INNER JOIN ");
        self.left.join_sql(out)
    }
}

impl<Left, Right> AsQuery for InnerJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
    (Left::AllColumns, Right::AllColumns): SelectableExpression<
                                   InnerJoinSource<Left, Right>,
                                   (Left::SqlType, Right::SqlType),
                               >,
{
    type SqlType = (Left::SqlType, Right::SqlType);
    type Query = SelectStatement<
        (Left::SqlType, Right::SqlType),
        (Left::AllColumns, Right::AllColumns),
        Self,
    >;

    fn as_query(self) -> Self::Query {
        SelectStatement::simple((Left::all_columns(), Right::all_columns()), self)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct LeftOuterJoinSource<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> LeftOuterJoinSource<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        LeftOuterJoinSource {
            left: left,
            right: right,
        }
    }
}

impl<Left, Right> QuerySource for LeftOuterJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
{
    fn from_clause(&self, out: &mut QueryBuilder) -> BuildQueryResult {
        try!(self.left.from_clause(out));
        out.push_sql(" LEFT OUTER JOIN ");
        self.left.join_sql(out)
    }
}

impl<Left, Right> AsQuery for LeftOuterJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
    (Left::AllColumns, Right::AllColumns): SelectableExpression<
                                   LeftOuterJoinSource<Left, Right>,
                                   (Left::SqlType, Nullable<Right::SqlType>),
                               >,
{
    type SqlType = (Left::SqlType, Nullable<Right::SqlType>);
    type Query = SelectStatement<
        (Left::SqlType, Nullable<Right::SqlType>),
        (Left::AllColumns, Right::AllColumns),
        Self,
    >;

    fn as_query(self) -> Self::Query {
        SelectStatement::simple((Left::all_columns(), Right::all_columns()), self)
    }
}

/// Indicates that two tables can be used together in a JOIN clause.
/// Implementations of this trait will be generated for you automatically by
/// the [association annotations](FIXME: Add link) from codegen.
pub trait JoinTo<T: Table>: Table {
    #[doc(hidden)]
    fn join_sql(&self, out: &mut QueryBuilder) -> BuildQueryResult;
}
