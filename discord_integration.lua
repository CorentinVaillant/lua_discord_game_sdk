--print("[lua discord int] start of the execution")
local CLIENT_ID = 984015223872704522 --toremove
local current_os

--detection of the os 
local path_sep = package.config:sub(1, 1)
if path_sep == "\\" then
    current_os = "Windows"
else
    current_os = "Unix"
end

--default function
local function uninitialize_func()
    print("the discord lib has not be well initialized")
    return nil
end

local discord = {
    is_corectly_init = uninitialize_func,
    update_callback = uninitialize_func

}


if (nil == os.getenv("LD_LIBRARY_PATH")) then
    print("LD_LIBRARY_PATH \n\t-> setting it to ./discord_game_sdk/lib/x86_64")
    if current_os == "Windows" then
        os.execute("set LD_LIBRARY_PATH=./discord_game_sdk/lib/x86_64")
    else
        os.execute("export LD_LIBRARY_PATH='./discord_game_sdk/lib/x86_64'")
    end
    
end

local discord_integration

require "target.debug.liblua_discord_game_sdk"
if pcall(require,"target.debug.liblua_discord_game_sdk") then
    print("[lua discord int] Unix debug")
    discord_integration = require("target.debug.liblua_discord_game_sdk")
elseif pcall(require,"target.release.liblua_discord_game_sdk") then
    print("[lua discord int] Unix release")
    discord_integration = require("target.release.liblua_discord_game_sdk")
elseif pcall(require,"liblua_discord_game_sdk") then
    print("[lua discord int] Unix lib")
    discord_integration = require("liblua_discord_game_sdk")
elseif pcall(require,"lua_discord_game_sdk") then
    print("[lua discord int] Windows lib")
    discord_integration = require("lua_discord_game_sdk")
elseif pcall(require,"target.release.lua_discord_game_sdk") then
    print("[lua discord int] Windows release")
    discord_integration = require("target.release.lua_discord_game_sdk")
elseif pcall(require,"target.debug.lua_discord_game_sdk") then
    print("[lua discord int] Windows debug")
    discord_integration = require("target.debug.lua_discord_game_sdk")
else
    error("error could not find so",2)
end

discord_integration.start_discord_sdk(CLIENT_ID)
local activity = {
    state = "playing trumpet",
    -- details = "details",

    start_time = 0,
    -- end_time = 9223372036854775806,

    -- large_image_key = "echo",
    -- large_image_tooltip = "echo",
    -- small_image_key = "miamzelda",
    -- small_image_tooltip = "miamzelda",

    -- party_id = "123456789",
    -- party_amount = 1;
    -- party_capacity = 20,

    -- instance = true,
    -- match_secret = "match12345",
    -- join_secret = "join12345",
    -- spectate_secret = "spectate12345"

}
print("send_activity result "..discord_integration.send_activity(activity))
local do_it = false
while true do

end


return discord