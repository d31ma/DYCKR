pub struct TakenOn<'a> {
    pub service: String,
    pub service_source: &'a str,
    pub methods: Vec<(String, usize)>,
}
