use discord_game_sdk::*;
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH};

//discord event handler
struct DiscordEventHandler {}

impl Default for DiscordEventHandler {
    fn default() -> Self {
        DiscordEventHandler {}
    }
}

impl EventHandler for DiscordEventHandler {}

#[derive(Debug)]
enum DiscordFunc<'a> {
    ActivityFunc(&'a Activity),
}

//test function
pub fn hello_rust() {
    println!("[rust] hello");
}

pub fn now() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.subsec_micros().try_into().expect("hmmmm !")
}

pub fn start_discord_sdk(client_id: i64) -> Result<()> {
    let (result_sender, result_receiver) = channel::<std::result::Result<(), Error>>();

    std::thread::spawn(move || {
        let (sender, receiver) = channel::<DiscordFunc>();

        let discord: Discord<'_, DiscordEventHandler>; //création dans un thread

        match Discord::new(client_id) {
            Ok(d) => {
                discord = d;
                result_sender.send(Ok(())).unwrap_or_else(|_a| {
                    eprintln!("could not send Ok(())!");
                    ()
                });
            }
            Err(e) => {
                eprintln!("could not start discord api");
                result_sender.send(Err(e)).unwrap_or_else(|_a| {
                    eprintln!("could not send Err({e})!");
                    ()
                });
                return;
            }
        }
        println!(
            "[rust lib] api started with {} client id!",
            discord.client_id()
        );

        let mut activity = Activity::empty();
        discord.update_activity(activity
            .with_state("In Play Mode")
            .with_start_time(5)
            .with_details("Playing the Trumpet!")
            , |_, result| {
            if let Err(error) = result {
                eprintln!("failed to update activity: {}", error);
            }
        });

        loop {
            match receiver.recv() {
                Ok(f) => handle_discord_func(&discord, f)
                    .unwrap_or_else(|e| println!("error while handling : {e}")),
                Err(e) => println!("Error recv {}", e),
            }
        }
    });

    println!("[rust lib] discord api thread initialized");
    result_receiver.recv().unwrap()
}


fn handle_discord_func<'a, E>(discord: &Discord<'a, E>, f: DiscordFunc) -> Result<()> {
    match f {
        DiscordFunc::ActivityFunc(activity) => discord.update_activity(activity, |_, result| {
            if let Err(error) = result {
                eprintln!("failed to update activity: {}", error);
            }
        }),
    }

    Ok(())
}
