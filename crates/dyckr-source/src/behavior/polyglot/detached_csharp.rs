use super::{Module, csharp};

pub use dyckr::detached_csharp::DetachedCsharp;

pub trait DetachedCsharpExt
where
    Self: Sized,
{
    fn of(module: &Module) -> Result<DetachedCsharp, String> {
        DetachedCsharp::of_with(module, &|m| csharp::lower(m), &|a, b, c, d, e, f| {
            csharp::runtime(a, b, c, d, e, f)
        })
    }

    fn page_socket_runtime(path: &std::path::Path) -> String {
        DetachedCsharp::page_socket_runtime_with(path, &|a, b, c, d, e, f| {
            csharp::runtime(a, b, c, d, e, f)
        })
    }
}

impl DetachedCsharpExt for DetachedCsharp {}
