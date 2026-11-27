use crate::behavior_module::{Method, Module};
use crate::csharp_task_answer::CsharpTaskAnswer;
use crate::csharp_task_result::CsharpTaskResult;
use crate::prepared::Prepared;
use crate::route_types::ModelTable;


pub fn describe(module: &mut Module, models: &ModelTable) -> Result<(), String> {
        for declared in &mut module.types {
            crate::csharp_task_models::describe_methods(&mut declared.methods, models)?;
        }
        Ok(())
    }

pub fn describe_methods(methods: &mut [Method], models: &ModelTable) -> Result<(), String> {
        for method in methods {
            crate::csharp_task_models::describe_method(method, models)?;
        }
        Ok(())
    }

pub fn describe_method(method: &mut Method, models: &ModelTable) -> Result<(), String> {
        (!method.metadata.is_empty())
            .then_some(method)
            .map(|selected| crate::csharp_task_models::describe_result(selected, models))
            .transpose()
            .map(|_| ())
    }

pub fn describe_result(method: &mut Method, models: &ModelTable) -> Result<(), String> {
        CsharpTaskResult::model(&method.return_type, models)?
            .map(|answer| crate::csharp_task_models::describe_model(method, &answer))
            .transpose()
            .map(|_| ())
    }

pub fn describe_model(method: &mut Method, answer: &CsharpTaskAnswer) -> Result<(), String> {
        crate::csharp_task_models::require_exchange(method)?;
        method
            .metadata
            .insert("tacModelResult".to_owned(), answer.metadata());
        Ok(())
    }

pub fn require_exchange(method: &Method) -> Result<(), String> {
        if !method.asynchronous
            || method
                .metadata
                .get("httpExchange")
                .and_then(|exchange| exchange.get("generated"))
                .is_none()
        {
            return Err("C# annotated Task or ValueTask of a model, list or plain value currently requires a generated HTTPClient exchange; other DTO behaviors remain unsupported.".to_owned());
        }
        Ok(())
    }


pub fn lower_prepared(prepared: Prepared, models: &ModelTable, lower: &dyn Fn(&Module) -> Result<String, String>) -> Result<String, String> {
        match prepared {
            crate::prepared::Prepared::Erased(erased) => Ok(erased),
            crate::prepared::Prepared::Annotated(mut module) => {
                crate::csharp_task_models::describe(&mut module, models)?;
                lower(&module)
            }
        }
    }
