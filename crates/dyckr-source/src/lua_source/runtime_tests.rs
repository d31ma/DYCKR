use super::LuaProgram;

#[test]
fn lua_json_keeps_nulls_and_empty_objects_and_refuses_invalid_input() {
    let Some(printed) = LuaProgram::printed(&[
        include_str!("lua_json.lua"),
        include_str!("../../tests/fixtures/lua/json_contract.lua"),
    ]) else {
        return;
    };
    assert!(printed.contains("Lua JSON contracts passed"), "{printed}");
}

#[test]
fn lua_behavior_timeouts_use_wall_time_and_waits_do_not_spin() {
    let Some(printed) = LuaProgram::printed(&[
        include_str!("lua_json.lua"),
        include_str!("lua_behaviors.lua"),
        include_str!("../../tests/fixtures/behavior/lua_clock_contract.lua"),
    ]) else {
        return;
    };
    assert!(
        printed.contains("Lua behavior clock contracts passed"),
        "{printed}"
    );
}
