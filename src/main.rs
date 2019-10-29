use clap::{App, Arg, ArgGroup, SubCommand};
use git_author::{
    error::Error,
    git::{author::AuthorBuilder, *},
};

fn main() -> Result<(), Error> {
    let config_file_location_args = [
        Arg::with_name("local")
            .help("use local config file (default)")
            .required(false)
            .group("config")
            .long("local")
            .display_order(0),
        Arg::with_name("global")
            .help("use global config file")
            .group("config")
            .required(false)
            .long("global")
            .display_order(0),
    ];
    let config_file_location_arg_group = ArgGroup::with_name("HOGE")
        .required(false)
        .arg("local")
        .arg("global");

    let commiter_args = [
        Arg::with_name("commiter name")
            .requires("commiter email")
            .required(true)
            .display_order(0),
        Arg::with_name("commiter email")
            .required(true)
            .display_order(1),
    ];
    let author_args = [
        Arg::with_name("author name")
            .requires("author email")
            .required(false)
            .display_order(3),
        Arg::with_name("author email")
            .required(false)
            .display_order(4),
    ];

    let get_subcommand = SubCommand::with_name("get")
        .about("Get repository or global author (default)")
        .usage("git author (get) [FLAGS]")
        .args(&config_file_location_args)
        .group(config_file_location_arg_group.clone())
        .display_order(0);
    let set_subcommand = SubCommand::with_name("set")
        .about("Set repository or global author")
        //.usage("git author set [FLAGS] [new user.name(opt)] [new user.email(opt)]")
        .args(&config_file_location_args)
        .args(&author_args)
        .group(config_file_location_arg_group.clone())
        .display_order(1);
    let replace_subcommand = SubCommand::with_name("replace")
        .about("Replace author information.\n\
            The target is all commits made by the specified user on the current branch of the current repository.\n\
            If 'author name' and 'author email' are not specified, the current local author is used.")
        .display_order(2)
        .args(&commiter_args)
        .args(&author_args);

    let app = App::new("git-author")
        .version("1.0.0")
        .usage("git-author [SUBCOMMAND] [FLAGS]")
        .about("Get and set repository or global author")
        .args(&config_file_location_args)
        .group(config_file_location_arg_group)
        .subcommand(get_subcommand)
        .subcommand(set_subcommand)
        .subcommand(replace_subcommand);

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("set") {
        let config_file_location = get_config_file_location(matches.is_present("global"));
        let author = if let (Some(name), Some(email)) = (
            matches.value_of("author name"),
            matches.value_of("author email"),
        ) {
            AuthorBuilder::new().name(name).email(email).build()?
        } else {
            get_author(get_config_file_location(matches.is_present("global")))?
        };
        set_author(config_file_location.clone(), author.clone())?;
        println!("set {} author: {}", config_file_location, author);
    } else if let Some(ref _matches) = matches.subcommand_matches("replace") {
        let commiter = AuthorBuilder::new()
            .name(matches.value_of("author name").unwrap())
            .email(matches.value_of("author email").unwrap())
            .build()?;
        let author = if let (Some(name), Some(email)) = (
            matches.value_of("author name"),
            matches.value_of("author email"),
        ) {
            AuthorBuilder::new().name(name).email(email).build()?
        } else {
            get_author(get_config_file_location(matches.is_present("global")))?
        };

        replace(commiter.clone(), author.clone())?;
        println!("replace commiter from {} to {}", commiter, author);
    } else {
        let author = get_author(get_config_file_location(matches.is_present("global")))?;
        println!("{}", author);
    }

    Ok(())
}

fn get_config_file_location(is_global: bool) -> ConfigFileLocation {
    if is_global {
        ConfigFileLocation::Global
    } else {
        ConfigFileLocation::Local
    }
}
