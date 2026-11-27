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
pub mod reconnect_policy;
pub mod remote_origin;
pub mod stream_query;
pub mod stream_event_name;
pub mod php_doc;
pub mod without_bom;
pub mod web_event_sender;
pub mod web_event_models;
pub mod web_event_sources;
pub mod language_kinds;
pub mod event_method;
pub mod event_stream;
pub mod event_listener;
pub mod event_resource;
pub mod event_type;
pub mod event_timer;
pub mod content_digest;
pub mod html_escape;
pub mod content_security_policy;
pub mod speculation_rules;
pub mod view_transition_guard;
pub mod base64;
pub mod typescript_program;
pub mod publish;
pub mod gateway_limits;
pub mod role_folders;
pub mod class_file_name;
pub mod declaration;
pub mod annotation_target;
pub mod native_companion;
pub mod dotnet_model_languages;
pub mod fsharp_syntax;
pub mod lua_syntax;
pub mod yard;
pub mod behavior_literal;
pub mod event_module;
pub mod http_client_source;
pub mod shadow_lines;
pub mod lua_lexer;
pub mod lua_body_sanitizer;
pub mod lua_writer_mode;
pub mod lua_types;
pub mod lua_line;
pub mod lua_shadow;
pub mod lua_shadow_writer;
pub mod lua_models;
pub mod fsharp_lexer;
pub mod fsharp_block_kind;
pub mod fsharp_line_kind;
pub mod fsharp_field_need;
pub mod fsharp_type;
pub mod fsharp_attribute_line;
pub mod fsharp_parameters;
pub mod fsharp_type_name;
pub mod fsharp_type_form;
pub mod fsharp_record_fields;
pub mod fsharp_type_header;
pub mod fsharp_field_needs;
pub mod fsharp_source_line;
pub mod fsharp_open_block;
pub mod fsharp_body;
pub mod fsharp_lines;
pub mod fsharp_value;
pub mod fsharp_member_line;
pub mod fsharp_member_words;
pub mod fsharp_member_header;
pub mod fsharp_shadow;
pub mod fsharp_shadow_writer;
pub mod yon_marker;
pub mod go_marker_name;
pub mod ruby_lexer;
pub mod lexical;
pub mod source_mask;
pub mod cpp_member_kind;
pub mod cpp_type_text;
pub mod cpp_attribute;
pub mod cpp_token;
pub mod cpp_parameter;
pub mod cpp_lexer;
pub mod cpp_member;
pub mod cpp_class;
pub mod cpp_source;
