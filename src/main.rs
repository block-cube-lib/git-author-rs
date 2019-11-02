use clap::{App, Arg, ArgMatches, SubCommand};
use git_author::{
    error::Error,
    git::{self, Author, ConfigFileLocation},
};
use std::error::Error as _;

const OLD_AUTHOR_NAME_KEY: &str = "old author name";
const OLD_AUTHOR_EMAIL_KEY: &str = "old author email";
const NEW_AUTHOR_NAME_KEY: &str = "new author name";
const NEW_AUTHOR_EMAIL_KEY: &str = "new author email";

const ABOUT_APPLICATION_TEXT: &str = r#"
- You can get or set user.name and user.email at oece.
- You can replace the author of past commits.
"#;

const ABOUT_REPLACE_TEXT: &str =
    "The author or committer replaces the commit author and committer of \
     `old author name <old author email>` \
     with `new author name <new author email>`";

fn main() {
    let result = command();
    if let Err(e) = result {
        println!("{}", e);
        let mut source = e.source();
        while let Some(s) = source {
            println!("{}", s);
            source = s.source();
        }
    }
}

fn command() -> Result<(), Error> {
    let config_file_locations = vec![ConfigFileLocation::Global, ConfigFileLocation::Local];
    let config_file_location_name_and_helps: Vec<_> = config_file_locations
        .iter()
        .map(|c| (format!("{}", c), format!("use {} config file", c)))
        .collect();

    let config_file_location_args: Vec<_> = config_file_location_name_and_helps
        .iter()
        .map(|(name, help)| {
            Arg::with_name(&name)
                .help(&help)
                .required(false)
                .group("config")
                .long(&name)
                .display_order(0)
        })
        .collect();

    let commiter_args = [
        Arg::with_name(OLD_AUTHOR_NAME_KEY)
            .requires(OLD_AUTHOR_EMAIL_KEY)
            .required(true)
            .display_order(0),
        Arg::with_name(OLD_AUTHOR_EMAIL_KEY)
            .required(true)
            .display_order(1),
    ];
    let author_args = [
        Arg::with_name(NEW_AUTHOR_NAME_KEY)
            .requires(NEW_AUTHOR_EMAIL_KEY)
            .required(false)
            .display_order(3),
        Arg::with_name(NEW_AUTHOR_EMAIL_KEY)
            .required(false)
            .display_order(4),
    ];

    let get_subcommand = SubCommand::with_name("get")
        .about("get user.name and user.email")
        .usage("git author (get) [FLAGS]")
        .args(&config_file_location_args)
        .display_order(0);
    let set_subcommand = SubCommand::with_name("set")
        .about("set user.name and user.email")
        .args(&config_file_location_args)
        .args(&author_args)
        .display_order(1);
    let replace_subcommand = SubCommand::with_name("replace")
        .about(ABOUT_REPLACE_TEXT)
        .display_order(2)
        .args(&commiter_args)
        .args(&author_args);

    let app = App::new("git-author")
        .version("1.0.0")
        .usage("git-author [SUBCOMMAND] [FLAGS]")
        .about(ABOUT_APPLICATION_TEXT)
        .args(&config_file_location_args)
        .subcommand(get_subcommand)
        .subcommand(set_subcommand)
        .subcommand(replace_subcommand);

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("get") {
        get_author(&matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("set") {
        set_author(&matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("replace") {
        replace(&matches)?;
    } else {
        // get
        get_author(&matches)?;
    }

    Ok(())
}

fn get_config_file_location(matches: &ArgMatches) -> Option<ConfigFileLocation> {
    use ConfigFileLocation::*;
    if matches.is_present(Global.to_string()) {
        Some(ConfigFileLocation::Global)
    } else if matches.is_present(Local.to_string()) {
        Some(ConfigFileLocation::Local)
    } else {
        None
    }
}

/// display author
fn get_author(matches: &ArgMatches) -> Result<(), Error> {
    let config_file_location = get_config_file_location(&matches);
    let author = git::get_author(config_file_location)?;
    match (author.name(), author.email()) {
        (Some(name), Some(email)) => println!("{} <{}>", name, email),
        (Some(name), None) => println!("{} (email is empty)", name),
        (None, Some(email)) => println!("<{}> (name is empty)", email),
        (None, None) => println!("name and email are empty"),
    };
    Ok(())
}

fn set_author(matches: &ArgMatches) -> Result<(), Error> {
    match (
        matches.value_of(NEW_AUTHOR_NAME_KEY),
        matches.value_of(NEW_AUTHOR_EMAIL_KEY),
    ) {
        (Some(name), Some(email)) => {
            let config_file_location = if let Some(location) = get_config_file_location(&matches) {
                location
            } else {
                ConfigFileLocation::Local
            };
            let author = Author::new(Some(name), Some(email))?;
            git::set_author(config_file_location, author)?;
            println!("set {} author: {} <{}>", config_file_location, name, email);
            Ok(())
        }
        (name, email) => Err(Error::InvalidArguments(format!(
            "The following required arguments were not provided:\n{}{}",
            if name.is_none() {
                "<author name>\n"
            } else {
                ""
            },
            if email.is_none() {
                "<author email>"
            } else {
                ""
            },
        ))),
    }
}

fn replace(matches: &ArgMatches) -> Result<(), Error> {
    let (old_author, new_author) = match (
        matches.value_of(OLD_AUTHOR_NAME_KEY),
        matches.value_of(OLD_AUTHOR_EMAIL_KEY),
        matches.value_of(NEW_AUTHOR_NAME_KEY),
        matches.value_of(NEW_AUTHOR_EMAIL_KEY),
    ) {
        (
            Some(old_author_name),
            Some(old_auhor_email),
            Some(new_author_name),
            Some(new_author_email),
        ) => {
            let old_author = Author::new(Some(old_author_name), Some(old_auhor_email))?;
            let new_author = Author::new(Some(new_author_name), Some(new_author_email))?;
            (old_author, new_author)
        }
        (Some(old_author_name), Some(old_author_email), None, None) => {
            let old_author = Author::new(Some(old_author_name), Some(old_author_email))?;
            let new_author = git::get_author(None)?;
            (old_author, new_author)
        }
        (_, _, _, _) => {
            return Err(Error::InvalidArguments(
                "Arguments has empty parameter.".to_string(),
            ));
        }
    };

    println!("{} -> {}", old_author, new_author);
    let output = git::replace(old_author, new_author)?;
    println!("{}", output);

    Ok(())
}
