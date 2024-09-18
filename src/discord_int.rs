use discord_game_sdk::*;

use std::time::{SystemTime, UNIX_EPOCH};

struct MyEventHandler {}

//event handler

impl Default for MyEventHandler {
    fn default() -> Self {
        MyEventHandler {}
    }
}

impl EventHandler for MyEventHandler {}

pub fn hello_rust() {
    println!("[rust] hello");
}

pub fn _now() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);

    since_the_epoch.subsec_micros().try_into().expect("hmmmm !")
}


pub fn start_discord_sdk(client_id: i64) -> Result<()> {
     
        let mut _discord: Discord<'_, MyEventHandler>;
        
        //let _:std::result::Result<discord_game_sdk::Discord<'_, MyEventHandler>, discord_game_sdk::Error> = Discord::new(client_id);

        /*-----------------------

        match Discord::new(client_id){
            Ok(d) => discord = d,
            _ => {
                eprintln!("could not start discord api");
            }
            
        }
        
        *discord.event_handler_mut() = Some(MyEventHandler::default());

        discord.update_activity(
            &Activity::empty()
                .with_state("On Main Menu")
                .with_start_time(now()),
            |_discord, result| {
                if let Err(error) = result {
                    eprintln!("failed to update activity: {}", error);
                }
            },
        );

        loop {
            let _ =discord.run_callbacks();
        }
        ----------------------------*/
     
    
    Ok(())
}

//appel de l'api


