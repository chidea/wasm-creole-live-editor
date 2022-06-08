use sycamore_router::Route;

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/w/<path..>")]
    Wiki { path: Vec<String> },
    #[to("/help")]
    Help,
    #[not_found]
    NotFound,
}
