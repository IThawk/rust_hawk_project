pub mod redis;

pub struct DbConnection<C, T> {
    config: C,
    connection: T,
}

impl<C, T> DbConnection<C, T> {
    fn test(s: C, t: T) -> T {
        return t;
    }
}
