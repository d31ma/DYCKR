pub fn yon_marker_name(written: &str) -> bool {
    let name = written.replace('_', "").to_ascii_lowercase();
    matches!(
        name.as_str(),
        "service"
            | "store"
            | "client"
            | "delegate"
            | "converter"
            | "lifetime"
            | "needs"
            | "stream"
            | "trace"
            | "useguards"
            | "useinterceptors"
            | "usefilters"
            | "middleware"
            | "guard"
            | "interceptor"
            | "exceptionfilter"
            | "module"
            | "injectable"
            | "errorhandler"
            | "handleserror"
            | "controlleradvice"
            | "restcontrolleradvice"
            | "exceptionhandler"
            | "identityprovider"
    ) || [
        "requestroute",
        "responsestatus",
        "stream",
        "getroute",
        "postroute",
        "putroute",
        "patchroute",
        "deleteroute",
        "streamroute",
    ]
    .contains(&name.as_str())
        || ["public", "signedin", "allow"].contains(&name.as_str())
        || ["socketroute", "onopen", "onmessage", "onclose", "onerror"].contains(&name.as_str())
        || ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"]
            .iter()
            .any(|verb| verb.eq_ignore_ascii_case(&name))
}
