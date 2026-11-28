use dyckr::annotation_rule::AnnotationRules;

pub struct FsharpMarkers;

impl FsharpMarkers {
    pub fn strip(source: &str) -> String {
        dyckr::fsharp_markers::FsharpMarkers::strip(source, &|name| {
            AnnotationRules::find(name).is_some()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FsharpMarkers;

    #[test]
    fn tachyon_attributes_are_blanked_and_others_stay() {
        let source = "[<TacSomething>] let x = 1 [<Other>] let y = 2";
        let stripped = FsharpMarkers::strip(source);
        assert_eq!(stripped, "[<           >] let x = 1 [<     >] let y = 2");
    }
}
