pub trait Random<T> {
    fn get_random_one(source: Vec<T>) -> T;
}

pub trait Round<T> {
    fn get_random_one(source: Vec<T>, before: i32) -> T;
}

pub trait Connection<C, T> {
    fn connect_to_db(connection_config: C) -> Result<T, String>;
}
