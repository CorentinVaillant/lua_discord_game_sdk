# Readme
## Use case:
I started this lib to implement the discord game sdk on the video game **Bugscraper**. My code is probably crappy, but feel free to use has you want, if you have question about it feel free to mail me at `contact.corentin.vaillant@gmail.com`. </br>
If you decide to use it for one of your project, please informe me, I like to know if my code is use somewere else ☺️.

## Doc
There will be doc, for the moment refere to the discord game sdk doc, and try to make something with, I am still here to help

## Usage
Place the discord game sdk in a 'discord_game_sdk' folder at the root of the project before building. </br>
like that  :
```
├───discord_game_sdk
│   └───lib
│       ├───x86
│       └───x86_64
├───src
└───target
    └──release ...
```
On windows place the Lua54.dll at the root of the library project to bind the .dll file with .lua file.
## Building
like before place the discord game sdk in the discord_game_sdk file. Set the `DISCORD_GAME_SDK_PATH` environement variable at the discord_game_sdk path (or whatever else where you place a discord game sdk folder)

## Thanks
I would like to thanks Hanna who help me for this project, and Leo who made me make this project.