use leptos::prelude::*;
use leptos_router::hooks::use_location;

pub struct QUERY;

impl QUERY {
    pub const WHERE: &str = "where";
}

/* ========================================================== */
/*                        ✨ STRUCT ✨                         */
/* ========================================================== */

pub struct QueryUtils;

impl QueryUtils {
    pub fn extract(query_key: String) -> Memo<Option<String>> {
        let location = use_location();

        Memo::new(move |_| location.query.with(|q| q.get(&query_key).map(|s| s.to_string())))
    }
}
