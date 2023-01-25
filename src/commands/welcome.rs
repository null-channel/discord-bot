use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("welcome")
        .description("Welcome a user")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to welcome")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("message")
                .description("The message to send")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice_localized(
                    "Welcome to our cool server! Ask me if you need help",
                    "pizza",
                    [(
                        "de",
                        "Willkommen auf unserem coolen Server! Frag mich, falls du Hilfe brauchst",
                    )],
                )
                .add_string_choice_localized("Hey, do you want a coffee?", "coffee", [(
                    "de",
                    "Hey, willst du einen Kaffee?",
                )])
                .add_string_choice_localized(
                    "Welcome to the club, you're now a good person. Well, I hope.",
                    "club",
                    [(
                        "de",
                        "Willkommen im Club, du bist jetzt ein guter Mensch. Naja, hoffentlich.",
                    )],
                )
                .add_string_choice_localized(
                    "I hope that you brought a controller to play together!",
                    "game",
                    [("de", "Ich hoffe du hast einen Controller zum Spielen mitgebracht!")],
                )
        })
}