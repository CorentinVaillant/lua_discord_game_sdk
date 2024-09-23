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

    // let a = |s: &String| -> &str { s.as_str() };


// table_key_into_activity("state", &table, Activity::with_state,String::as_str, &mut activity);
table_key_into_activity(
    "state",
    &table,
    Activity::with_state,
    |s: &String| -> &str { s.as_str() },  // Explicit return type
    &mut activity
);

    

    match table.get::<&str,String>("state"){
        Ok(value) => {activity.with_state(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("details"){
        Ok(value) => {activity.with_details(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,i64>("start_time"){
        Ok(value) => {activity.with_start_time(value);},
        Err(_) => (),
    };

    match table.get::<&str,i64>("end_time"){
        Ok(value) => {activity.with_end_time(value);},
        Err(_) => (),
    };

    match table.get::<&str,String>("large_image_key"){
        Ok(value) => {activity.with_large_image_key(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("large_image_tooltip"){
        Ok(value) => {activity.with_large_image_tooltip(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("small_image_key"){
        Ok(value) => {activity.with_small_image_key(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("small_image_tooltip"){
        Ok(value) => {activity.with_small_image_tooltip(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("party_id"){
        Ok(value) => {activity.with_party_id(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,u32>("party_amount"){
        Ok(value) => {activity.with_party_amount(value);},
        Err(_) => (),
    };

    match table.get::<&str,u32>("party_capacity"){
        Ok(value) => {activity.with_party_capacity(value);},
        Err(_) => (),
    };

    match table.get::<&str,bool>("instance"){
        Ok(value) => {activity.with_instance(value);},
        Err(_) => (),
    };

    match table.get::<&str,String>("match_secret"){
        Ok(value) => {activity.with_match_secret(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("join_secret"){
        Ok(value) => {activity.with_join_secret(value.as_str());},
        Err(_) => (),
    };

    match table.get::<&str,String>("spectate_secret"){
        Ok(value) => {activity.with_spectate_secret(value.as_str());},
        Err(_) => (),
    };

    match update_activity(activity){
        Ok(()) => Ok(0),
        Err(e) => Err(LuaError::external(e)),
    }

}

fn table_key_into_activity<'a,TableType,ActivityFuncType,F,H>(key :&str, table : &'a LuaTable,activity_method :F,table_type_to_activity_func_type :H,activity:&mut Activity)
where F : Fn(&mut Activity, ActivityFuncType) -> &mut Activity ,
TableType: mlua::FromLua<'a>,
H : Fn(&TableType)->ActivityFuncType{
    let f = table_type_to_activity_func_type;

    match table.get::<&str,TableType>(key){
        Ok(value) => {activity_method(activity,f(&value));},
        Err(_) => (),
    };
    
}