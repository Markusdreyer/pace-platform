use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Stuff<'a> {
    first: &'a str,
    last: &'a str,
}
