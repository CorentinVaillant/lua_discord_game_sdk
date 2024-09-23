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

fn lua_send_activity<'a>(_lua: &Lua, table: LuaTable) -> LuaResult<LuaInteger> {
    let mut activity = Activity::empty();

    table_key_string_into_activity("state", &table, Activity::with_state, &mut activity);
    table_key_string_into_activity("details", &table, Activity::with_details, &mut activity);

    table_key_into_activity("start_time", &table, Activity::with_start_time, &mut activity);

    

    match table.get::<&str, i64>("end_time") {
        Ok(value) => {
            activity.with_end_time(value);
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("large_image_key") {
        Ok(value) => {
            activity.with_large_image_key(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("large_image_tooltip") {
        Ok(value) => {
            activity.with_large_image_tooltip(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("small_image_key") {
        Ok(value) => {
            activity.with_small_image_key(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("small_image_tooltip") {
        Ok(value) => {
            activity.with_small_image_tooltip(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("party_id") {
        Ok(value) => {
            activity.with_party_id(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, u32>("party_amount") {
        Ok(value) => {
            activity.with_party_amount(value);
        }
        Err(_) => (),
    };

    match table.get::<&str, u32>("party_capacity") {
        Ok(value) => {
            activity.with_party_capacity(value);
        }
        Err(_) => (),
    };

    match table.get::<&str, bool>("instance") {
        Ok(value) => {
            activity.with_instance(value);
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("match_secret") {
        Ok(value) => {
            activity.with_match_secret(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("join_secret") {
        Ok(value) => {
            activity.with_join_secret(value.as_str());
        }
        Err(_) => (),
    };

    match table.get::<&str, String>("spectate_secret") {
        Ok(value) => {
            activity.with_spectate_secret(value.as_str());
        }
        Err(_) => (),
    };

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
