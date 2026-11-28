#![allow(clippy::expect_used)]

use std::path::Path;

#[test]
fn ordinary_javascript_service_classes_are_not_inline_models() {
    let path = Path::new("client/services/InputService.js");
    let source =
        "@Service\nexport default class InputService { accept(value) { return value; } }\n";
    assert!(
        dyckr::model_blocks::ModelBlocks::read(path, source)
            .expect("ordinary source")
            .is_empty()
    );
    crate::annotation_layout::InlineModelCheck::refuse_inline_models(path, source)
        .expect("service stays a service");
}
