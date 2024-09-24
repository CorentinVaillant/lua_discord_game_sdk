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

discord_integration.start_discord_sdk(CLIENT_ID)
discord_integration.send_activity({
    state = "state",
    details = "details",

    start_time = 0,
    end_time = 9223372036854775806,

    large_image_key = "echo",
    large_image_tooltip = "echo",
    small_image_key = "miamzelda",
    small_image_tooltip = "miamzelda",

    party_id = "123456789",
    party_amount = 1;
    party_capacity = 20,

    instance = true,
    match_secret = "match12345",
    join_secret = "join12345",
    spectate_secret = "spectate12345"


})

print("envoie fait")

while true do
    os.execute("sleep " .. tonumber(1))
    print("wsh")
    discord_integration.update_callback()
end


return discord