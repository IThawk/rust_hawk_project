use crate::algorithm::random::make_random_string;

const UUID_LENGTH: i32 = 32;
const REVISION_LENGTH: i32 = 16;

pub fn get_uuid() -> String {
    make_random_string(UUID_LENGTH)
}

pub fn get_revision() -> String {
    make_random_string(REVISION_LENGTH)
}
