pub use dyckr::java_runtime_section::JavaRuntimeSection;

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::JavaRuntimeSection;

    fn section(entries: &[(&str, &str)]) -> JavaRuntimeSection {
        let mut section = JavaRuntimeSection::default();
        for (label, text) in entries {
            section.add((*label).to_owned(), (*text).to_owned());
        }
        section
    }

    #[test]
    fn equal_entries_of_two_services_are_kept_once() {
        let first = section(&[
            ("Shared", "case \"Shared\": a;"),
            ("Own", "case \"Own\": b;"),
        ]);
        let second = section(&[
            ("Shared", "case \"Shared\": a;"),
            ("Other", "case \"Other\": c;"),
        ]);
        let joined = first.joined(&second).expect("no conflict");
        assert_eq!(
            joined.text(),
            "case \"Shared\": a;case \"Own\": b;case \"Other\": c;"
        );
        assert_eq!(
            joined.labels().collect::<Vec<_>>(),
            ["Shared", "Own", "Other"]
        );
    }

    #[test]
    fn one_label_with_two_texts_cannot_join() {
        let first = section(&[("Input", "case \"Input\": a;")]);
        let second = section(&[("Input", "case \"Input\": b;")]);
        assert!(first.joined(&second).is_none());
    }

    #[test]
    fn an_empty_entry_is_not_kept() {
        assert_eq!(section(&[("Plain", "")]).labels().count(), 0);
    }
}
