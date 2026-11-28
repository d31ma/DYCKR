use crate::csharp_task_answer::CsharpTaskAnswer;
use crate::route_types::ModelTable;

pub struct CsharpTaskResult;

impl CsharpTaskResult {
    pub const TASKS: [&'static str; 4] = [
        "Task<",
        "System.Threading.Tasks.Task<",
        "ValueTask<",
        "System.Threading.Tasks.ValueTask<",
    ];
    pub const TASK_BASES: [&'static str; 4] = [
        "Task",
        "System.Threading.Tasks.Task",
        "ValueTask",
        "System.Threading.Tasks.ValueTask",
    ];
    pub const LISTS: [&'static str; 2] = ["List<", "System.Collections.Generic.List<"];
    pub const ARRAY: &'static str = "[]";
    pub const SCALARS: [(&'static str, &'static str); 7] = [
        ("string", "string"),
        ("int", "integer"),
        ("long", "integer"),
        ("double", "number"),
        ("float", "number"),
        ("decimal", "number"),
        ("bool", "boolean"),
    ];
    pub const REFUSAL: &'static str = "A C# browser result is Task or ValueTask of one registered client/models record, a List or array of registered records or plain values, or one plain value (string, int, long, double, float, decimal or bool); maps, nested lists, nullable results and other types are not supported.";

    pub fn model(kind: &str, models: &ModelTable) -> Result<Option<CsharpTaskAnswer>, String> {
        Self::require_task(kind.trim())?;
        Self::registered(kind.trim(), models)
    }

    pub fn require_task(kind: &str) -> Result<(), String> {
        let malformed = [Self::task_base(kind), Self::argument(kind).is_none()];
        (malformed != [true, true])
            .then_some(())
            .ok_or_else(|| Self::REFUSAL.to_owned())
    }

    pub fn task_base(kind: &str) -> bool {
        Self::TASK_BASES.contains(&kind.split('<').next().unwrap_or(kind))
    }

    pub fn argument(kind: &str) -> Option<&str> {
        Self::TASKS
            .iter()
            .find_map(|task| kind.strip_prefix(task))?
            .strip_suffix('>')
            .map(str::trim)
    }

    pub fn registered(kind: &str, models: &ModelTable) -> Result<Option<CsharpTaskAnswer>, String> {
        Self::argument(kind)
            .filter(|item| *item != "BehaviorJSON")
            .map(|item| Self::answer(item, models))
            .transpose()
    }

    pub fn answer(payload: &str, models: &ModelTable) -> Result<CsharpTaskAnswer, String> {
        let array = payload.strip_suffix(Self::ARRAY);
        let list = Self::list_item(payload);
        let item = array.or(list);
        let single = Self::item(item.unwrap_or(payload), models)?;
        let forms: [fn(CsharpTaskAnswer, bool) -> CsharpTaskAnswer; 2] = [
            Self::single,
            crate::csharp_task_answer::CsharpTaskAnswer::listed,
        ];
        Ok(forms[usize::from(item.is_some())](single, array.is_some()))
    }

    pub fn single(answer: CsharpTaskAnswer, _array: bool) -> CsharpTaskAnswer {
        answer
    }

    pub fn list_item(payload: &str) -> Option<&str> {
        Self::LISTS
            .iter()
            .find_map(|list| payload.strip_prefix(list))
            .and_then(|rest| rest.strip_suffix('>'))
            .map(str::trim)
    }

    pub fn item(item: &str, models: &ModelTable) -> Result<CsharpTaskAnswer, String> {
        Self::SCALARS
            .iter()
            .find(|(name, _)| *name == item)
            .map_or_else(
                || Self::record(item, models),
                |(name, shape)| {
                    Ok(crate::csharp_task_answer::CsharpTaskAnswer::value(
                        name, shape,
                    ))
                },
            )
    }

    pub fn record(item: &str, models: &ModelTable) -> Result<CsharpTaskAnswer, String> {
        let name = item.strip_prefix("Models.").unwrap_or(item);
        if !models.is_record(name) {
            return Err(format!(
                "C# browser Task result '{item}' requires a registered client/models record, a List or array of them, or a plain value; maps, nested lists and unknown types are unsupported."
            ));
        }
        models.check_all()?;
        let schema = models.event_models(name)?;
        crate::csharp_task_shape::CsharpTaskShape::validate(name, &schema)?;
        Ok(crate::csharp_task_answer::CsharpTaskAnswer::record(
            name, schema,
        ))
    }
}
