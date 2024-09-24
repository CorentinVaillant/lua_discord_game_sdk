mod discord_int;

use discord_game_sdk::Activity;
use discord_int::*;
use mlua::prelude::*;

//same name as the module file(with path)

#[mlua::lua_module]
fn target_debug_liblua_discord_game_sdk(lua: &Lua) -> LuaResult<LuaTable> {
    let result = lua_init(lua).expect("something went wrong");

    Ok(result)
}

#[mlua::lua_module]
fn liblua_discord_game_sdk(lua: &Lua) -> LuaResult<LuaTable> {
    let result = lua_init(lua).expect("something went wrong");

    Ok(result)
}

#[mlua::lua_module]
fn target_release_liblua_discord_game_sdk(lua: &Lua) -> LuaResult<LuaTable> {
    lua_init(lua)
}

fn lua_init(lua: &Lua) -> LuaResult<LuaTable> {
    println!(
        "[lib] running for {}-{}-{} ",
        std::env::consts::ARCH,
        std::env::consts::FAMILY,
        std::env::consts::OS
    );

    let export = lua.create_table()?;

    export.set(
        "start_discord_sdk",
        lua.create_function(lua_start_discord_sdk)?,
    )?;

    export.set(
        "update_callback",
        lua.create_function(lua_discord_update_callback)?,
    )?;

    export.set("send_activity", lua.create_function(lua_send_activity)?)?;

    println!("[lib] :export DONE");
    Ok(export)
}

fn lua_start_discord_sdk(_lua: &Lua, id: LuaInteger) -> LuaResult<LuaInteger> {
    let _ = start_discord_sdk(id);
    Ok(0)
}

fn lua_discord_update_callback(_lua: &Lua, _: ()) -> LuaResult<LuaInteger> {
    match update_callback() {
        Ok(_) => Ok(0),
        Err(e) => {
            eprintln!("{e}");
            Err(LuaError::external(e))
        }
    }
}

fn lua_table_to_activity(table: &LuaTable) -> Activity{
    let mut activity = Activity::empty();

    table_key_string_into_activity("state", &table, Activity::with_state, &mut activity);
    table_key_string_into_activity("details", &table, Activity::with_details, &mut activity);

    table_key_into_activity("start_time", &table, Activity::with_start_time, &mut activity);
    table_key_into_activity("end_time", &table, Activity::with_end_time, &mut activity);
    
    table_key_string_into_activity("large_image_key",&table, Activity::with_large_image_key, &mut activity);
    table_key_string_into_activity("large_image_tooltip",&table, Activity::with_large_image_tooltip, &mut activity);
    table_key_string_into_activity("small_image_key",&table, Activity::with_small_image_key, &mut activity);
    table_key_string_into_activity("small_image_tooltip",&table, Activity::with_small_image_tooltip, &mut activity);

    table_key_string_into_activity("party_id",&table, Activity::with_party_id, &mut activity);
    table_key_into_activity("party_amount", &table, Activity::with_party_amount, &mut activity);
    table_key_into_activity("party_capacity", &table, Activity::with_party_capacity, &mut activity);

    table_key_into_activity("instance", &table, Activity::with_instance, &mut activity);
    table_key_string_into_activity("match_secret",&table, Activity::with_match_secret, &mut activity);
    table_key_string_into_activity("join_secret",&table, Activity::with_join_secret, &mut activity);
    table_key_string_into_activity("spectate_secret",&table, Activity::with_spectate_secret, &mut activity);

    activity


}

fn lua_send_activity<'a>(_lua: &Lua, table: LuaTable) -> LuaResult<LuaInteger> {
    
    let activity = lua_table_to_activity(&table);


    match update_activity(activity) {
        Ok(()) => Ok(0),
        Err(e) => Err(LuaError::external(e)),
    }
}

fn table_key_string_into_activity<'a, 'b, F>(
    key: &str,
    table: &'a LuaTable,
    activity_method: F,
    activity: &'b mut Activity,
) where
    F: Fn(&'b mut Activity, &str) -> &'b mut Activity,
{
    match table.get::<&str, String>(key) {
        Ok(value) => {
            activity_method(activity, value.as_str());
        }
        Err(_) => (),
    };
}

fn table_key_into_activity<'a, T, F>(
    key: &str,
    table: &'a LuaTable,
    activity_method: F,
    activity: &mut Activity,
) where
    F: Fn(&mut Activity, T) -> &mut Activity,
    T: FromLua<'a>,
{
    match table.get::<&str, T>(key) {
        Ok(value) => {
            activity_method(activity, value);
        }
        Err(_) => (),
    };
}
