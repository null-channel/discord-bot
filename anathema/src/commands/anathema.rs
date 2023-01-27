use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use rusqlite::{Connection, Result};
use crate::db::user::{get_user, User, upsert_user, insert_user};


pub fn run(options: &[CommandDataOption], conn: Connection) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let CommandDataOptionValue::User(discord_user, _member) = option else {
        return "Please provide a valid user".to_string();
    };

    // do database things here
    let user_ret = get_user(&conn, discord_user.id.0);

    let Ok(user) = user_ret else {    
        let new_user_value = User {
            id: discord_user.id.0,
            name: discord_user.name.clone(),
            anathema: 0 + 1,
            credits: 10,
        }; 

        let _user = insert_user(&conn, new_user_value.clone());

        return format!("New user {}'s anathema is {}", new_user_value.name, new_user_value.anathema)
    };

    let new_user_value = User {
        id: user.id,
        name: user.name,
        anathema: user.anathema + 1,
        credits: user.credits,
    }; 

    let _user = upsert_user(&conn, new_user_value.clone());

    format!("{}'s anathema is {}", new_user_value.name, new_user_value.anathema)

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("anathema").description("Get a user anathema").create_option(|option| {
        option
            .name("anathema")
            .description("The user to lookup")
            .kind(CommandOptionType::User)
            .required(true)
    })
}