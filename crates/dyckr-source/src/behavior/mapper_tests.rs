#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::behavior::lower;
    use std::path::Path;

    fn lower_tac_js(source: &str) -> Result<Option<String>, crate::Failure> {
        lower(Path::new("tac.js"), source, "tac")
    }

    fn source(plan: &str) -> String {
        format!(
            "@Converter()\nclass ProductMapper {{\n@Convert({plan})\nasync toSummary(product) {{}}\n}}\n"
        )
    }

    #[test]
    fn a_mapping_plan_reaches_the_registration() {
        let plan = "{\"fields\":[{\"source\":\"detail.id\",\"target\":\"productId\"},{\"target\":\"kind\",\"constant\":\"product\"}]}";
        let output = lower_tac_js(&source(plan))
            .expect("valid mapping")
            .expect("lowered");
        assert!(output.contains("\"mapping\""), "{output}");
        assert!(output.contains("detail.id"), "{output}");
    }

    #[test]
    fn the_one_parameter_is_the_converted_value() {
        let plan = "{\"fields\":[{\"source\":\"id\",\"target\":\"id\"}]}";
        let output = lower_tac_js(&source(plan))
            .expect("valid mapping")
            .expect("lowered");
        assert!(output.contains("\"argument\":0"), "{output}");
    }

    #[test]
    fn invalid_plans_fail_closed() {
        for plan in [
            "{\"fields\":[]}",
            "{\"fields\":[{\"target\":\"id\"}]}",
            "{\"fields\":[{\"target\":\"id\",\"source\":\"a\",\"constant\":1}]}",
            "{\"fields\":[{\"target\":\"\",\"source\":\"a\"}]}",
            "{\"fields\":[{\"target\":\"id\",\"source\":\"a..b\"}]}",
            "{\"fields\":[{\"target\":\"id\",\"source\":\"a.b.c.d.e.f.g.h.i\"}]}",
            "{\"fields\":[{\"target\":\"id\",\"source\":\"a\"},{\"target\":\"id\",\"source\":\"b\"}]}",
            "{\"fields\":[{\"target\":\"id\",\"constant\":{}}]}",
            "{\"fields\":[{\"target\":\"id\",\"source\":\"a\",\"extra\":1}]}",
            "{\"argument\":0,\"fields\":[{\"target\":\"id\",\"source\":\"a\"}]}",
        ] {
            assert!(lower_tac_js(&source(plan)).is_err(), "accepted {plan}");
        }
    }

    #[test]
    fn convert_takes_one_json_parameter() {
        let plan = "{\"fields\":[{\"source\":\"id\",\"target\":\"id\"}]}";
        let two = format!(
            "@Converter()\nclass ProductMapper {{\n@Convert({plan})\nasync toSummary(product, other) {{}}\n}}\n"
        );
        let failure = lower_tac_js(&two).expect_err("two parameters");
        assert!(
            failure.to_string().contains("exactly one parameter"),
            "{failure}"
        );
        let model = format!(
            "@Converter()\nclass ProductMapper {{\n@Convert({plan})\nasync toSummary(product: Product): Promise<unknown> {{}}\n}}\n"
        );
        let failure = lower(Path::new("tac.ts"), &model, "tac").expect_err("model parameter");
        assert!(failure.to_string().contains("'Product'"), "{failure}");
        let old = "{\"argument\":0,\"fields\":[{\"source\":\"id\",\"target\":\"id\"}]}";
        let failure = lower_tac_js(&source(old)).expect_err("argument key");
        assert!(
            failure.to_string().contains("no \"argument\" key"),
            "{failure}"
        );
    }

    #[test]
    fn a_mapping_method_writes_no_body() {
        let occupied = "@Converter()\nclass ProductMapper {\n@Convert({\"fields\":[{\"source\":\"id\",\"target\":\"id\"}]})\nasync toSummary(product) { return product; }\n}\n";
        assert!(lower_tac_js(occupied).is_err());
    }
}
