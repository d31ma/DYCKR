pub struct RustTaskMember;

impl RustTaskMember {
    pub const FALLIBLE_FORMS: [(&'static str, &'static str); 2] =
        [("BehaviorResult<", ">"), ("Result<", ",BehaviorError>")];
    pub const NO_ANSWER: &'static str = "()";
    pub const AWAITED: [&'static str; 2] = ["", ".await"];

    pub fn crosses(returns: Option<&str>) -> bool {
        let (_, answer) = Self::answer(returns);
        [
            answer == Self::NO_ANSWER,
            crate::rust_model_type::RustModelType::crosses(&answer),
        ]
        .contains(&true)
    }

    pub fn fallible(returns: Option<&str>) -> bool {
        Self::answer(returns).0
    }

    pub fn entry(
        (key, name): (&str, &str),
        returns: Option<&str>,
        parameters: &[String],
        asynchronous: bool,
    ) -> String {
        let (bindings, passed) = crate::rust_arguments::rust_arguments(parameters);
        let (fallible, answer) = Self::answer(returns);
        let awaited = Self::AWAITED[usize::from(asynchronous)];
        let call = format!("service.{name}({passed}){awaited}");
        let value = Self::value(fallible, &answer, &call);
        format!(
            "(\"{key}\", TacMember::Task({{let it = instance.clone(); Box::new(move |arguments: Vec<TacValue>| {{ #[allow(unused_mut, unused_variables)] let mut arguments = arguments.into_iter(); {bindings}let owner = it.clone(); Box::pin(async move {{ #[allow(unused_mut)] let mut service = owner.try_borrow_mut().map_err(|_| TacTaskError::busy())?; {value} }}) }})}})),"
        )
    }

    fn value(fallible: bool, answer: &str, call: &str) -> String {
        if fallible {
            return format!(
                "match {call} {{ Ok(value) => Ok({}), Err(error) => Err(TacTaskError {{ code: error.code, message: error.message }}) }}",
                crate::rust_model_type::rust_read(answer, "value")
            );
        }
        format!("Ok({})", crate::rust_model_type::rust_read(answer, call))
    }

    fn answer(returns: Option<&str>) -> (bool, String) {
        let compact = returns
            .unwrap_or(Self::NO_ANSWER)
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        Self::FALLIBLE_FORMS
            .iter()
            .find_map(|(start, end)| compact.strip_prefix(start)?.strip_suffix(end))
            .map_or_else(
                || (false, compact.clone()),
                |inner| (true, inner.to_owned()),
            )
    }
}
