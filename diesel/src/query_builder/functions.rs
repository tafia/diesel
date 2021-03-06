use super::{UpdateTarget, IncompleteUpdateStatement};
use super::delete_statement::DeleteStatement;
use super::IncompleteInsertStatement;

/// Creates an update statement. Helpers for updating a single row can be
/// generated by
/// [`#[changeset_for]`](https://github.com/sgrif/diesel/tree/master/diesel_codegen#changeset_fortable_name).
///
/// # Example
///
/// # Examples
///
/// ### Deleting a single record:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("src/doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Serial,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     use self::users::dsl::*;
/// #     use diesel::query_builder::update;
/// #     let connection = establish_connection();
/// let command = update(users.filter(id.eq(1)))
///     .set(name.eq("James"));
/// let updated_row = connection.query_one(command);
/// // When passed to `query_one`, the update statement will gain `RETURNING *`
/// assert_eq!(Ok((1, "James".to_string())), updated_row);
/// # }
/// ```
pub fn update<T: UpdateTarget>(source: T) -> IncompleteUpdateStatement<T> {
    IncompleteUpdateStatement::new(source)
}

/// Creates a delete statement. Will delete the records in the given set.
/// Because this function has a very generic name, it is not exported by
/// default.
///
/// # Examples
///
/// ### Deleting a single record:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("src/doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Serial,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     delete();
/// # }
/// #
/// # fn delete() -> QueryResult<()> {
/// #     use self::users::dsl::*;
/// #     use diesel::query_builder::delete;
/// #     let connection = establish_connection();
/// #     let get_count = || users.count().first::<i64>(&connection).unwrap();
/// let old_count = get_count();
/// try!(delete(users.filter(id.eq(1))).execute(&connection));
/// assert_eq!(old_count - 1, get_count());
/// # Ok(())
/// # }
/// ```
///
/// ### Deleting a whole table:
///
/// ```rust
/// # #[macro_use] extern crate diesel;
/// # include!("src/doctest_setup.rs");
/// #
/// # table! {
/// #     users {
/// #         id -> Serial,
/// #         name -> VarChar,
/// #     }
/// # }
/// #
/// # fn main() {
/// #     delete();
/// # }
/// #
/// # fn delete() -> QueryResult<()> {
/// #     use self::users::dsl::*;
/// #     use diesel::query_builder::delete;
/// #     let connection = establish_connection();
/// #     let get_count = || users.count().first::<i64>(&connection).unwrap();
/// try!(delete(users).execute(&connection));
/// assert_eq!(0, get_count());
/// # Ok(())
/// # }
/// ```
pub fn delete<T: UpdateTarget>(source: T) -> DeleteStatement<T> {
    DeleteStatement::new(source)
}

/// Creates an insert statement. Will add the given data to a table. This
/// function is not exported by default. As with other commands, the resulting
/// query can return the inserted rows if you choose.
pub fn insert<T>(records: T) -> IncompleteInsertStatement<T> {
    IncompleteInsertStatement::new(records)
}
