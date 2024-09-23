use discord_game_sdk::*;
use std::sync::mpsc::{channel, Sender};
use std::sync::OnceLock;

static DISCORD_FUNC_SENDER: OnceLock<Sender<DiscordFunc>> = OnceLock::new();

//discord event handler
#[derive(Default)]
struct DiscordEventHandler {}

impl EventHandler for DiscordEventHandler {}

#[derive(Debug)]
#[allow(dead_code)]
enum DiscordFunc {
    ActivityFunc(Activity),
    RunCallBack,
}

pub fn start_discord_sdk(client_id: i64) -> Result<()> {
    let (result_sender, result_receiver) = channel::<std::result::Result<(), Error>>();

    //return if already been init
    if DISCORD_FUNC_SENDER.get().is_some() {
        eprintln!("[rust lib] the lib has already been started");
        return Ok(());
    }

    std::thread::spawn(move || {
        let (sender, receiver) = channel::<DiscordFunc>();

        match DISCORD_FUNC_SENDER.set(sender) {
            Ok(_) => (),
            Err(e) => println!("[rust lib] could not initialized DISCORD_FUNC_SENDER :{e:?}"),
        };

        let mut discord: Discord<'_, DiscordEventHandler>;

        match Discord::new(client_id) {
            Ok(d) => {
                discord = d;
                result_sender.send(Ok(())).unwrap_or_else(|_a| {
                    eprintln!("[rust lib] could not send Ok(())!");
                });
            }
            Err(e) => {
                eprintln!("[rust lib] could not start discord api");
                result_sender.send(Err(e)).unwrap_or_else(|_a| {
                    eprintln!("[rust lib] could not send Err({e})!");
                });
                return;
            }
        }
        println!(
            "[rust lib] api started with {} client id!",
            discord.client_id()
        );

        loop {
            match receiver.recv() {
                Ok(f) => handle_discord_func(&mut discord, f)
                    .unwrap_or_else(|e| println!("error while handling : {e}")),
                Err(e) => println!("[rust lib] Error recv {}", e),
            }
        }
    });

    println!("[rust lib] discord api thread initialized");
    let a = result_receiver.recv_timeout(std::time::Duration::from_secs(2));
    match a {
        Ok(result) => result,
        Err(e) => {
            eprintln!("[rust lib]Error :{e}");
            Ok(())
        }
    }
}

fn handle_discord_func<E>(discord: &mut Discord<'_, E>, f: DiscordFunc) -> Result<()> {
    match f {
        DiscordFunc::ActivityFunc(activity) => discord.update_activity(&activity, |_, result| {
            if let Err(error) = result {
                eprintln!("failed to update activity: {}", error);
            }
        }),
        DiscordFunc::RunCallBack => discord.run_callbacks()?,
    }

    Ok(())
}

//send DiscordFunc::RunCallBack into DISCORD_FUNC_SENDER
pub fn update_callback() -> std::result::Result<(), String> {
    match DISCORD_FUNC_SENDER.get() {
        Some(sender) => match sender.send(DiscordFunc::RunCallBack) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        None => Err("could not send".to_string()),
    }
}

pub fn update_activity(activity: Activity) -> std::result::Result<(), String> {
    match DISCORD_FUNC_SENDER.get() {
        Some(sender) => match sender.send(DiscordFunc::ActivityFunc(activity)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        None => Err("could not send".to_string()),
    }
}
