use anyhow::{anyhow, Result};
use clap::Parser;
use git_author::{
    error::*,
    git::{self, Author, ConfigFileLocation, ReplaceFilter, ReplaceTarget},
};

const NAME_KEY: &str = "name";
const EMAIL_KEY: &str = "email";


fn main() -> Result<()> {
    let git_author = git_author::command::GitAuthor::parse();
    git_author.execute_subcommand()?;

    //replace::option::init()?;

    //let result = command();
    //match result {
    //    Ok(_) => Ok(()),
    //    Err(e) => {
    //        println!("{}", e);
    //        let mut source = e.source();
    //        while let Some(s) = source {
    //            println!("{}", s);
    //            source = s.source();
    //        }
    //        Err(e.into())
    //    }
    //}
    Ok(())
}

/*
fn command() -> Result<()> {
    let config_file_location_name_and_helps: Vec<_> = ConfigFileLocation::VARIANTRS
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

    let get_subcommand = SubCommand::with_name("get")
        .about("get user.name and user.email")
        .usage("git author (get) [FLAGS]")
        .args(&config_file_location_args)
        .display_order(0);

    let set_subcommand = {
        let author_args = [
            Arg::with_name(NAME_KEY)
                .requires(EMAIL_KEY)
                .empty_values(false)
                .required(true)
                .display_order(1),
            Arg::with_name(EMAIL_KEY).required(true).display_order(2),
        ];
        SubCommand::with_name("set")
            .about("set user.name and user.email")
            .args(&config_file_location_args)
            .args(&author_args)
            .display_order(1)
    };

    let unset_subcommand = SubCommand::with_name("unset")
        .about("unset user.name and user.email")
        .usage("git author unset [FLAGS]")
        .args(&config_file_location_args)
        .display_order(2);

    // git author replace
    let replace_subcommand = {
        use replace::*;

        let simple_subcommand = {
            use option::simple::*;

            let args = [
                Arg::with_name(OLD_NAME_KEY)
                    .requires(OLD_EMAIL_KEY)
                    .empty_values(false)
                    .required(true)
                    .display_order(1),
                Arg::with_name(OLD_EMAIL_KEY)
                    .required(true)
                    .display_order(2),
                Arg::with_name(NEW_NAME_KEY)
                    .requires(NEW_EMAIL_KEY)
                    .empty_values(false)
                    .required(false)
                    .display_order(3),
                Arg::with_name(NEW_EMAIL_KEY)
                    .required(false)
                    .display_order(4),
            ];

            SubCommand::with_name(NAME)
                .args(&args)
                .about(&**option::simple::ABOUT.get().unwrap())
                .display_order(1)
        };

        let detail_subcommand = {
            use option::detail::*;
            let filter_author = Arg::with_name(FILTER_AUTHOR)
                .long(FILTER_AUTHOR)
                .value_names(&[&NAME_KEY, &EMAIL_KEY])
                .empty_values(false)
                .required_unless(FILTER_COMMITTER)
                .help(&FILTER_AUTHOR_HELP.get().unwrap())
                .display_order(0);
            let filter_committer = Arg::with_name(FILTER_COMMITTER)
                .long(FILTER_COMMITTER)
                .value_names(&[&NAME_KEY, &EMAIL_KEY])
                .empty_values(false)
                .required_unless(FILTER_AUTHOR)
                .help(&FILTER_COMMITTER_HELP.get().unwrap())
                .display_order(1);
            let filter_type = Arg::with_name(FILTER_TYPE)
                .long(FILTER_TYPE)
                .takes_value(true)
                .default_value(FILTER_AUTHOR_AND_COMMITTER)
                .empty_values(false)
                .help(&FILTER_TYPE_HELP.get().unwrap())
                .display_order(2);

            let replace_author = Arg::with_name(AUTHOR)
                .long(AUTHOR)
                .value_names(&[&NAME_KEY, &EMAIL_KEY])
                .empty_values(false)
                .help(AUTHOR_HELP)
                .display_order(3);
            let replace_committer = Arg::with_name(COMMITTER)
                .long(COMMITTER)
                .value_names(&[&NAME_KEY, &EMAIL_KEY])
                .empty_values(false)
                .help(COMMITTER_HELP)
                .display_order(4);
            let replace_target = Arg::with_name(REPLACE_TARGET)
                .long(REPLACE_TARGET)
                .help(&TARGET_HELP.get().unwrap())
                .required(true)
                .takes_value(true)
                .empty_values(false)
                .display_order(5);

            SubCommand::with_name(NAME)
                .display_order(2)
                .arg(filter_author)
                .arg(filter_committer)
                .arg(replace_author)
                .arg(replace_committer)
                .arg(replace_target)
                .arg(filter_type)
        };

        SubCommand::with_name("replace")
            .about(ABOUT)
            .display_order(3)
            .subcommand(simple_subcommand)
            .subcommand(detail_subcommand)
    };

    let app = App::new("git-author")
        .version(crate_version!())
        .usage("git-author [SUBCOMMAND] [FLAGS]")
        .about(crate_description!())
        .args(&config_file_location_args)
        .subcommand(get_subcommand)
        .subcommand(set_subcommand)
        .subcommand(unset_subcommand)
        .subcommand(replace_subcommand);

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("get") {
        get_author(&matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("set") {
        set_author(&matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("unset") {
        unset_author(&matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("replace") {
        replace::replace(&matches)?;
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

fn unset_author(matches: &ArgMatches) -> Result<(), Error> {
    let config_file_location = get_config_file_location(&matches);
    git::unset_author(config_file_location)?;
    Ok(())
}

mod replace {
    pub mod option {
        pub fn init() -> anyhow::Result<()> {
            detail::init()?;
            simple::init()?;

            Ok(())
        }

        pub mod detail {
            use once_cell::sync::OnceCell;
            pub const NAME: &str = "detail";

            pub const FILTER_TYPE: &str = "filter-type";
            pub const FILTER_AUTHOR: &str = "filter-author";
            pub const FILTER_COMMITTER: &str = "filter-committer";
            pub const FILTER_AUTHOR_OR_COMMITTER: &str = "or";
            pub const FILTER_AUTHOR_AND_COMMITTER: &str = "and";

            pub const REPLACE_TARGET: &str = "replace-target";
            pub const REPLACE_TARGET_AUTHOR: &str = "author";
            pub const REPLACE_TARGET_COMMITTER: &str = "committer";
            pub const REPLACE_TARGET_AUTHOR_AND_COMMITTER: &str = "author-and-committer";

            pub const AUTHOR: &str = "author";
            pub const COMMITTER: &str = "committer";
            pub static AUTHOR_HELP: &str = "author after replacement. \
                 If not specified, use author which can be obrtained by `git author get`";
            pub static COMMITTER_HELP: &str = "committer after replacement. \
                 If not specified, use author which can be obrtained by `git author get`";

            pub static FILTER_AUTHOR_HELP: OnceCell<String> = OnceCell::new();
            pub static FILTER_COMMITTER_HELP: OnceCell<String> = OnceCell::new();
            pub static FILTER_TYPE_HELP: OnceCell<String> = OnceCell::new();
            pub static TARGET_HELP: OnceCell<String> = OnceCell::new();

            pub fn init() -> anyhow::Result<()> {
                use anyhow::anyhow;
                FILTER_AUTHOR_HELP
                    .set(format!(
                        "filter with author. Required when `{}` is not specified.",
                        FILTER_COMMITTER
                    ))
                    .map_err(|e| anyhow!("set is not success: {}", e))?;
                FILTER_COMMITTER_HELP
                    .set(format!(
                        "filter with committer. Required when `{}` is not specified.",
                        FILTER_AUTHOR
                    ))
                    .map_err(|e| anyhow!("set is not success: {}", e))?;
                FILTER_TYPE_HELP
                    .set(format!(
                        "You can specify `{and}` or `{or}`. \
                     Valid only both `{filter_author}` and `{filter_committer}` are specified. \
                     It is ignored at other times.\n\
                     The defalut is `{and}`.\n\
                     If `{and}` is specified, \
                     commits that match `author` and `committer` are specified, \
                     and if `{or}` is specified, \
                     commits that match either `author` or `committer` are included.",
                        and = FILTER_AUTHOR_AND_COMMITTER,
                        or = FILTER_AUTHOR_OR_COMMITTER,
                        filter_author = FILTER_AUTHOR,
                        filter_committer = FILTER_COMMITTER
                    ))
                    .map_err(|e| anyhow!("set is not success: {}", e))?;
                TARGET_HELP
                    .set(format!(
                        "Replacement target. You can specify `{}` or`{}` or `{}`.",
                        REPLACE_TARGET_AUTHOR,
                        REPLACE_TARGET_COMMITTER,
                        REPLACE_TARGET_AUTHOR_AND_COMMITTER
                    ))
                    .map_err(|e| anyhow!("set is not success: {}", e))?;

                Ok(())
            }
        }

        pub mod simple {
            use anyhow::anyhow;
            use once_cell::sync::OnceCell;

            pub const NAME: &str = "simple";
            pub const OLD_NAME_KEY: &str = "old-name";
            pub const OLD_EMAIL_KEY: &str = "old-email";
            pub const NEW_NAME_KEY: &str = "new-name";
            pub const NEW_EMAIL_KEY: &str = "new-email";

            pub static ABOUT: OnceCell<String> = OnceCell::new();

            pub fn init() -> anyhow::Result<()> {
                ABOUT
                    .set(format!(
                        "Replace the Author or Committer's `{}` with `{}` and \
                     `{}` with `{}` in the past commit.",
                        OLD_NAME_KEY, OLD_EMAIL_KEY, NEW_NAME_KEY, NEW_EMAIL_KEY
                    ))
                    .map_err(|e| anyhow!("set is not success: {}", e))?;

                Ok(())
            }
        }
    }

    pub const ABOUT: &str = "The author or committer replaces the commit author and committer of \
                             `old author name <old author email>` \
                             with `new author name <new author email>`";

    use super::*;

    pub fn replace(matches: &ArgMatches) -> Result<()> {
        if let Some(ref matches) = matches.subcommand_matches(option::simple::NAME) {
            replace_simple(&matches)?;
        } else if let Some(ref matches) = matches.subcommand_matches(option::detail::NAME) {
            replace_detail(&matches)?;
        }
        Ok(())
    }

    fn replace_detail(matches: &ArgMatches) -> Result<()> {
        let filter = parse_filter(&matches)?;
        match &filter {
            ReplaceFilter::AuthorOnly(author) => println!("filter author: {}", author),
            ReplaceFilter::CommitterOnly(committer) => println!("filter committer: {}", committer),
            ReplaceFilter::AuthorOrCommitter { author, committer } => println!(
                "filter author or committer\n\
                 author   : {}\n\
                 committer: {}",
                author, committer
            ),
            ReplaceFilter::AuthorAndCommitter { author, committer } => println!(
                "filter author and committer\n\
                 author   : {}\n\
                 committer: {}",
                author, committer
            ),
        };

        let target = parse_target(&matches)?;
        match &target {
            ReplaceTarget::Author { new_author } => println!("new author: {}", new_author),
            ReplaceTarget::Committer { new_committer } => {
                println!("new committer: {}", new_committer)
            }
            ReplaceTarget::AuthorAndCommitter {
                new_author,
                new_committer,
            } => println!(
                "new author   : {}\n\
                 new committer: {}",
                new_author, new_committer
            ),
        }

        git::replace(filter, target)?;

        Ok(())
    }

    fn replace_simple(matches: &ArgMatches) -> Result<()> {
        use option::simple::*;

        let old_name = matches.value_of(OLD_NAME_KEY);
        let old_email = matches.value_of(OLD_EMAIL_KEY);
        let old_author = Author::new(old_name, old_email)?;

        let new_author = match (
            matches.value_of(NEW_NAME_KEY),
            matches.value_of(NEW_EMAIL_KEY),
        ) {
            (None, None) => git::get_author(None)?,
            (name, email) => Author::new(name, email)?,
        };

        git::replace_simple(old_author, new_author).map_err(|e| e.into())
    }

    // Option<Values> to Result<Option<Author>, Error>
    fn values_to_author(values: Option<clap::Values>) -> Result<Option<Author>> {
        if let Some(mut values) = values {
            let author = Author::new(values.next(), values.next())?;
            Ok(Some(author))
        } else {
            Ok(None)
        }
    }

    fn parse_filter(matches: &ArgMatches) -> Result<ReplaceFilter> {
        use option::detail::*;

        let author = values_to_author(matches.values_of(FILTER_AUTHOR))?;
        let committer = values_to_author(matches.values_of(FILTER_COMMITTER))?;
        let filter_type = matches.value_of(FILTER_TYPE);
        let filter = match (author, committer, filter_type) {
            (Some(author), None, _) => ReplaceFilter::AuthorOnly(author),
            (None, Some(committer), _) => ReplaceFilter::CommitterOnly(committer),
            (Some(author), Some(committer), Some(FILTER_AUTHOR_AND_COMMITTER)) => {
                ReplaceFilter::AuthorAndCommitter { author, committer }
            }
            (Some(author), Some(committer), Some(FILTER_AUTHOR_OR_COMMITTER)) => {
                ReplaceFilter::AuthorOrCommitter { author, committer }
            }
            (_, _, _) => {
                return Err(InvalidArguments(format!(
                    "The only value that can be specified for filter-type is `{}` or `{}`.",
                    FILTER_AUTHOR_AND_COMMITTER, FILTER_AUTHOR_OR_COMMITTER,
                ))
                .into())
            }
        };
        Ok(filter)
    }

    fn parse_target(matches: &ArgMatches) -> Result<ReplaceTarget> {
        use option::detail::*;

        let author = values_to_author(matches.values_of(AUTHOR))?.unwrap_or(git::get_author(None)?);
        let committer =
            values_to_author(matches.values_of(COMMITTER))?.unwrap_or(git::get_author(None)?);
        let replace_target = matches.value_of(REPLACE_TARGET);
        let target = match replace_target {
            Some(REPLACE_TARGET_AUTHOR) => ReplaceTarget::Author { new_author: author },
            Some(REPLACE_TARGET_COMMITTER) => ReplaceTarget::Committer {
                new_committer: committer,
            },
            Some(REPLACE_TARGET_AUTHOR_AND_COMMITTER) => ReplaceTarget::AuthorAndCommitter {
                new_author: author,
                new_committer: committer,
            },
            _ => {
                return Err(InvalidArguments(format!(
                    "The only value that can be specified for filter-type is `{}` or `{}` or `{}`.",
                    REPLACE_TARGET_AUTHOR,
                    REPLACE_TARGET_COMMITTER,
                    REPLACE_TARGET_AUTHOR_AND_COMMITTER
                ))
                .into())
            }
        };
        Ok(target)
    }
}
*/
