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
pub mod cpp_failure;
pub mod cpp_tokens;
pub mod cpp_attributes;
pub mod cpp_member_end;
pub mod cpp_shape;
pub mod cpp_step;
pub mod cpp_declaration;
pub mod cpp_member_reader;
pub mod cpp_file_reader;
pub mod cpp_file;
pub mod cpp_yon_models;
pub mod declarations;
pub mod route_types;
pub mod go_source;
pub mod style_path;
pub mod vendor_provenance;
pub mod style_source;
pub mod config_failure;
pub mod blob_limits;
pub mod browser_api_grants;
pub mod storage_member;
pub mod quoted_string_escape;
pub mod safety;
pub mod yon_member;
pub mod native_outgoing_hosts;
pub mod peers_member;
pub mod native_theme_color;
pub mod media_member;
pub mod native_display;
pub mod native_orientation;
pub mod native_title_bar;
pub mod native_title_bar_button_inset;
pub mod native_menu_shortcut;
pub mod system_version;
pub mod rust_dependency;
pub mod native_menu_action;
pub mod valid_route;
pub mod native_menu_entry;
pub mod xml_escape;
pub mod browser_api_permissions;
pub mod native_links;
pub mod target_floor;
pub mod minimum_system;
pub mod native_route_index;
pub mod native_menu_config;
pub mod valid_icon_source;
pub mod native_tray;
pub mod rust_companion_crate;
pub mod native_tool_failure;
pub mod rust_native_libraries;
pub mod rust_companion_output;
pub mod build_profile;
pub mod linkage;
pub mod rust_wasm_cache;
pub mod cache;
pub mod native_rust_project;
pub mod native_config;
pub mod native_site;
pub mod behavior_input;
pub mod behavior_marker;
pub mod gateway;
pub mod hot_update;
pub mod migration;
pub mod piece;
pub mod java_model_schema;
pub mod companion_language;
pub mod instance_name;
pub mod native_application_lifetime;
pub mod swift_platform;
pub mod kotlin_builtins;
pub mod companion_kind;
pub mod model_blocks;
pub mod java_builtins;
pub mod lower_first;
pub mod declares_table;
pub mod modifiers;
pub mod companion_syntax;
pub mod kotlin_header;
pub mod companion_failures;
pub mod go;
pub mod companion_names;
pub mod kotlin_member;
pub mod rust_model_type;
pub mod declaration_names;
pub mod declared_service;
pub mod owner_name;
pub mod companion_converters;
pub mod constructor_field_policy;
pub mod rust_collaborators;
pub mod kotlin_member_syntax;
pub mod csharp_model_type;
pub mod rust_arguments;
pub mod rust_task_member;
pub mod companion_member;
pub mod parameter_text;
pub mod csharp_browser_members;
pub mod rust_construction;
pub mod rust_held_fields;
pub mod rust_owner_table;
pub mod rust_graph;
pub mod class_table;
pub mod behavior_module;
pub mod prepared;
pub mod provider_qualifiers;
pub mod kotlin_members;
pub mod with_required_constructor;
