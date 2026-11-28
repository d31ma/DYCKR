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
        let source = "[<Service>]\ntype A() =\n    [<GetRoute; Obsolete>]\n    member this.Get([<FromBody>] order: Order) = \"[<Service>]\"\n";
        let stripped = FsharpMarkers::strip(source);
        assert_eq!(
            stripped,
            "           \ntype A() =\n    [<Obsolete>]          \n    member this.Get(             order: Order) = \"[<Service>]\"\n"
        );
    }
}
