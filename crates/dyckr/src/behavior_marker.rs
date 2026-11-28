use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Marker {
    pub name: String,
    pub arguments: String,
    pub span: Range<usize>,
    pub target: usize,
    pub parameter: bool,
}

impl Marker {
    pub const RETIRED_NAMES: [(&str, &str); 4] = [
        ("Cache", "CacheSettings"),
        ("SpringBootTest", "Test"),
        ("Pageable", "PageDefaults"),
        ("EventPublisher", "an injected ApplicationEventPublisher"),
    ];
    pub const SLICE_ADVICE: &str = "write Test: there is no ambient context to slice, because an invocation owns its own instance graph and a Setup is always selected explicitly";
    pub const SPY_ADVICE: &str =
        "there is no bean to wrap; write Fake and assign the stub the test wants";
    pub const UNTESTABLE_NAMES: [(&str, &str); 8] = [
        ("WebMvcTest", Self::SLICE_ADVICE),
        ("WebFluxTest", Self::SLICE_ADVICE),
        ("DataJpaTest", Self::SLICE_ADVICE),
        ("JsonTest", Self::SLICE_ADVICE),
        ("RestClientTest", Self::SLICE_ADVICE),
        (
            "AutoConfigureMockMvc",
            "call the controller's verb method directly; there is no MockMvc because the route directory, not an annotation, owns the URL",
        ),
        ("SpyBean", Self::SPY_ADVICE),
        ("MockitoSpyBean", Self::SPY_ADVICE),
    ];
}
