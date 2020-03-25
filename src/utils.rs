pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_json, Result as JsonResult};

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub kind: &'static str,
    pub value: String,
}

impl From<Result<String, String>> for Output {
    fn from(res: Result<String, String>) -> Output {
        match res {
            Ok(value) => Output { kind: "ok", value },
            Err(value) => Output { kind: "err", value },
        }
    }
}
