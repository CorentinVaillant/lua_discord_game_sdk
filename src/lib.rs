mod discord_int;

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

    println!("[lib] running for {}-{}-{} ",std::env::consts::ARCH,std::env::consts::FAMILY,std::env::consts::OS);

    let export = lua.create_table()?;
    export.set("hello_rust",lua.create_function(|_lua, _: ()| {hello_rust();Ok(())})?)?;
    //if i comment the following line lua will detect the .so file
    export.set("start_discord_sdk", lua.create_function(|_lua: &Lua,id|{lua_start_discord_sdk(_lua,id)?;Ok(())})?)?;

    println!("[lib] : DONE");
    Ok(export)
}


fn lua_start_discord_sdk(_lua :&Lua,id :LuaInteger)->LuaResult<LuaInteger>{
    println!("[lib :lua_start_discord_sdk] starting");
    let _ = start_discord_sdk(id);

    Ok(0)
}

