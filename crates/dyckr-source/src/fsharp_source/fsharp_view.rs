pub use dyckr::fsharp_view::FsharpView;

#[cfg(test)]
mod tests {
    use super::FsharpView;
    use crate::Failure;
    use crate::annotation_rules::AnnotationRulesExt;
    use dyckr::failure::{diagnostic, source_span};
    use std::path::Path;

    #[test]
    fn a_failure_on_the_shadow_points_at_the_fsharp_file() {
        let path = Path::new("server/services/TallyService.fs");
        let source = "[<Service>]\ntype TallyService() =\n    member this.Count() : int = 1\n";
        let view = FsharpView::of(path, source).unwrap_or_else(|| panic!("fsharp"));
        assert_eq!(view.path(), Path::new("server/services/TallyService.cs"));
        let at = view
            .text()
            .find("Count")
            .unwrap_or_else(|| panic!("member"));
        let failure = Failure::one(diagnostic(
            1002,
            "Bad member in 'server/services/TallyService.cs'",
            None,
            source_span("server/services/TallyService.cs", at, at + 5),
        ));
        let mapped = view.failure(&failure);
        let mapped = &mapped.diagnostics()[0];
        assert_eq!(
            mapped.message,
            "Bad member in 'server/services/TallyService.fs'"
        );
        assert_eq!(mapped.spans[0].file, "server/services/TallyService.fs");
        let start =
            usize::try_from(mapped.spans[0].start).unwrap_or_else(|error| panic!("start: {error}"));
        let line_start = source[..start].rfind('\n').map_or(0, |at| at + 1);
        assert!(source[line_start..].starts_with("    member this.Count"));
    }

    #[test]
    fn a_refusal_on_the_shadow_speaks_fsharp() {
        let path = Path::new("client/services/ChatService.fs");
        let source =
            "[<RequestRoute(\"/chat\")>]\ntype ChatService() =\n    member this.Run() = ()\n";
        let message = dyckr::annotation_rule::AnnotationRules::check(path, source)
            .err()
            .unwrap_or_else(|| panic!("F# Tac has no routes"))
            .to_string();
        assert!(
            message.contains("[<RequestRoute>] is not used in Tac code"),
            "{message}"
        );
        assert!(
            message.contains("client/services/ChatService.fs"),
            "{message}"
        );
        assert!(!message.contains("C#"), "{message}");
    }

    #[test]
    fn a_tac_socket_client_passes_the_check() {
        let path = Path::new("client/remotes/ChatClient.fs");
        let source = "[<HTTPClient(\"/chat\")>]\ntype ChatClient =\n    [<SocketRequest>]\n    abstract Receive: Tick -> unit\n";
        let checked = dyckr::annotation_rule::AnnotationRules::check(path, source);
        assert!(
            checked.is_ok(),
            "{:?}",
            checked.err().map(|error| error.to_string())
        );
    }

    #[test]
    fn other_languages_have_no_view() {
        assert!(FsharpView::of(Path::new("server/services/A.cs"), "class A {}").is_none());
    }
}
