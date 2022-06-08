use sycamore_router::Route;

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/w/<path..>")]
    Wiki { path: Vec<String> },
    #[to("/e/<path..>")]
    WikiEdit { path: Vec<String> },
    #[to("/d/<path..>")]
    WikiDelete { path: Vec<String> },
    #[to("/help")]
    Help,
    #[not_found]
    NotFound,
}
