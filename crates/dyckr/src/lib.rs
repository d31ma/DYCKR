pub mod ascii_name;
pub mod external_command;
pub mod sorted_json;
pub mod unicode_name;
pub mod portable_path;
pub mod failure;
pub mod node_driver;
pub mod python_parameters;
pub mod wasm_optimizer;
pub mod ttid;
pub mod web_api_clients;
pub mod helper_new_call;
pub mod class_name_from_file;
pub mod groovy_source;
pub mod kotlin_home;

#[macro_export]
macro_rules! one_test_per_case {
    ($module:ident, $check:ident, all $all:expr, [$($name:ident => $case:expr),+ $(,)?]) => {
        mod $module {
            #[allow(unused_imports)]
            use super::*;
            $(
                #[test]
                fn $name() { $check($case); }
            )+
            #[test]
            fn every_case_has_its_own_test() {
                let mut tested = vec![$($case),+];
                let mut all: Vec<_> = $all.into_iter().collect();
                tested.sort(); all.sort();
                assert_eq!(tested, all, "every case needs its own test");
            }
        }
    };
    ($module:ident, async $check:ident, all $all:expr, [$($name:ident => $case:expr),+ $(,)?]) => {
        mod $module {
            #[allow(unused_imports)]
            use super::*;
            $(
                #[tokio::test]
                async fn $name() { $check($case).await; }
            )+
            #[test]
            fn every_case_has_its_own_test() {
                let mut tested = vec![$($case),+];
                let mut all: Vec<_> = $all.into_iter().collect();
                tested.sort(); all.sort();
                assert_eq!(tested, all, "every case needs its own test");
            }
        }
    };
}
pub mod route_access;
pub mod identifier;
pub mod topic_access;
pub mod realm;
pub mod program_probe;
pub mod probe_memory;
pub mod topics;
pub mod topic_table;
pub mod routing;
pub mod lexical_mask;
pub mod cpp_raw_text;
pub mod framework_marker;
pub mod renamed;
pub mod shape;
pub mod model;
pub mod type_text;
pub mod plan;
