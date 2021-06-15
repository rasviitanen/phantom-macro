


/// Used in macros to expand token trees without using/including
/// the invariant we are expanding over.
///
/// It works by peeling the first argument and then returning the remaining tokens.
///
/// # Example
/// ```rust
/// use phantom_macro::expand;
///
/// macro_rules! csv {
///     [$($item: literal),* $(,)?] => {
///         format!(
///             concat!($(expand!($item => "{},")),*),
///             $($item),*
///         )
///     }
/// }
///
/// fn main() {
///     let csv = csv!["a", "b", "c"];
/// }
/// ```
#[macro_export]
macro_rules! expand {
    ($_: tt => { $($rest: tt)* }) => { $($rest)* };
    ($_: tt => $($rest: tt)*) => { $($rest)* };
}