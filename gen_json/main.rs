use serde::{Serialize, Deserialize};
use serde_json::ser;
use std::fs::File;
#[derive(Serialize, Deserialize)]
pub struct Entry {
    message: &'static str,
    description: &'static str,
}
macro_rules! register_long_diagnostics {
    ($($code:ident: $value:expr),*) => {
        #[allow(unused)]
        pub fn map() -> std::collections::BTreeMap<&'static str, crate::Entry> {
            let mut map = std::collections::BTreeMap::new();
            $(
                map.insert(stringify!($code), crate::Entry { description: stringify!($code), message: $value });
            )*
            map
        }
    };
    ($($code:tt: $description:tt),*,) => {
        register_long_diagnostics!{$($code: $description),*}
    }
}
macro_rules! register_diagnostics {
    ($($code:tt),*) => {};
    ($($code:tt),*,) => {};
}

include!("modules.rs");

fn main() {
    let mut list = Vec::new();
    include!{"register.rs"}
    for (name, map) in list {
        let f = File::create(format!("../locales/en-US/{}.json", name)).unwrap();
        ser::to_writer_pretty(f, &map).unwrap();
    }
}

