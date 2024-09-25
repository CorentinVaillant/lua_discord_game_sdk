# Readme
## Use case:
I started this lib to implement the Discord Game SDK for the video game **[Bugscraper](https://github.com/Yolwoocle/bugscraper)**. My code is probably crappy, but feel free to use it as you want, if you have questions about it feel free to email me at `contact.corentin.vaillant@gmail.com`. 

If you decide to use it for one of your projects, please inform me, I'd like to know if my code is used somewhere else ☺️.

## Doc
There will be docs, for the moment please refer to the [Discord Game SDK docs](https://discord.com/developers/docs/developer-tools/game-sdk), and try to make something with it, I am still here to help.

## Usage
Place the Discord Game SDK in a 'discord_game_sdk' folder at the root of the project before building. </br>
Like that:
```
├───discord_game_sdk
│   └───lib
│       ├───x86
│       └───x86_64
├───src
└───target
    └──release ...
```
On Windows place the `Lua54.dll` at the root of the library project to bind the `.dll` file with `.lua` file.

## Building
Like before place the Discord Game SDK in the `discord_game_sdk` file. Set the `DISCORD_GAME_SDK_PATH` environement variable at the `discord_game_sdk` path (or wherever else you place the Discord Game SDK folder)

## Thanks
I would like to thank Hana who helped me for this project, and Léo who made me make this project.
