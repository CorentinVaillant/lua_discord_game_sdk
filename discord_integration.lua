--print("[lua discord int] start of the execution")
local CLIENT_ID = 1114962997060849774 --toremove


local function uninitialize_func()
    print("the discord lib has not be well initialized")
    return nil
end

local discord = {
    is_corectly_init = uninitialize_func,
    update_callback = uninitialize_func

}


if (nil == os.getenv("LD_LIBRARY_PATH")) then
    print("please set LD_LIBRARY_PATH environment variable")
    return discord
end

local discord_integration
discord_integration = require("target.debug.liblua_discord_game_sdk")
if pcall(require,"target.debug.liblua_discord_game_sdk") then
    print("[lua discord int] debug")
    discord_integration = require("target.debug.liblua_discord_game_sdk")

elseif pcall(require,"target.release.liblua_discord_game_sdk") then
    print("[lua discord int] release")
    discord_integration = require("target.release.liblua_discord_game_sdk")

elseif pcall(require,"liblua_discord_game_sdk") then
    print("[lua discord int] lib")
    discord_integration = require("liblua_discord_game_sdk")
else
    error("error could not find so",2)
end

print("caca\n.\n.")
discord_integration.start_discord_sdk(CLIENT_ID)
print("caca\n\n")
discord_integration.start_discord_sdk(CLIENT_ID)

while true do
    io.write(".")
    io.write(string.char(10))
end


return discord