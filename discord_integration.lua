--print("[lua discord int] start of the execution")

local CLIENT_ID = 1114962997060849774 --toremove

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
discord_integration.hello_rust()


while true do
    print("aaa")
end


print("[lua discord int] end of execution")