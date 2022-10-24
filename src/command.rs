use crate::{
    email_address::EmailAddress,
    git::{self, Author, ConfigFileLocation, ReplaceTarget},
};
use anyhow::Result;
use clap::*;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
pub struct GitAuthor {
    #[clap(subcommand)]
    command: Commands,
}

impl GitAuthor {
    pub fn execute_subcommand(self) -> Result<()> {
        self.command.execute()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(
        group(
            ArgGroup::new("config_file_location")
                .required(false)
                .args(&["local", "global"])),
        display_order(0),
        about("Get user.name and user.email"))]
    Get {
        #[clap(long, display_order(0))]
        local: bool,
        #[clap(long, display_order(1))]
        global: bool,
    },

    #[clap(
        group(
            ArgGroup::new("config_file_location")
                .required(false)
                .args(&["local", "global"])),
        display_order(1),
        about("Set user.name and user.email"))]
    Set {
        name: String,
        email: EmailAddress,
        #[clap(long, display_order(0))]
        local: bool,
        #[clap(long, display_order(1))]
        global: bool,
    },

    #[clap(
        group(
            ArgGroup::new("config_file_location")
                .required(false)
                .args(&["local", "global"])),
        display_order(2),
        about("Unset user.name and user.email"))]
    Unset {
        #[clap(long, display_order(0))]
        local: bool,
        #[clap(long, display_order(1))]
        global: bool,
    },
    // git author replace a a@email.com [b b@email.com]
    // git author replace --committer-only a a@email.com [b b@email.com]
    // git author replace --author-only    a a@email.com [b b@email.com]
    #[clap(
        group(ArgGroup::new("new_author").required(false)),
        display_order(3),
        about("Replace the author or committer of past commits")
    )]
    Replace {
        old_name: String,
        old_email: EmailAddress,
        #[clap(requires("new-email"))]
        new_name: Option<String>,
        new_email: Option<EmailAddress>,
        #[clap(arg_enum, long, default_value = "author-and-committer")]
        replace_target: ReplaceTarget,
        //replace_filter: ReplaceFilter,
        //replace_target: ReplaceTarget,
    },
}

impl Commands {
    pub fn execute(&self) -> Result<()> {
        use Commands::*;
        use ConfigFileLocation::*;
        match self {
            Get { local, global } => Self::get(match (local, global) {
                (true, _) => Some(Local),
                (_, true) => Some(Global),
                _ => None,
            }),
            Set {
                local,
                global,
                name,
                email,
            } => Self::set(
                &name,
                email.clone(),
                match (local, global) {
                    (true, _) => Some(Local),
                    (_, true) => Some(Global),
                    _ => None,
                },
            ),
            Unset { local, global } => Self::unset(match (local, global) {
                (true, _) => Some(Local),
                (_, true) => Some(Global),
                _ => None,
            }),
            Replace {
                old_name,
                old_email,
                new_name,
                new_email,
                replace_target,
            } => {
                //let filter = match (author_only, committer_only) =>/ ReplaceFilter::
                let r = Replace {
                    old_name: old_name.clone(),
                    old_email: old_email.clone(),
                    new_name: new_name.clone(),
                    new_email: new_email.clone(),
                    replace_target: replace_target.clone(),
                };
                dbg!(r);
                let old_author = Author::new(Some(old_name), Some(old_email));
                let new_author = if let (Some(new_name), Some(new_email)) = (new_name, new_email) {
                    Author::new(new_name.as_ref(), new_email)
                } else {
                    Self::get(None)
                };
                Ok(())
            }
        }
    }

    fn get(config_file_location: Option<ConfigFileLocation>) -> Result<()> {
        let author = git::get_author(config_file_location)?;
        match (author.name(), author.email()) {
            (Some(name), Some(email)) => println!("{} <{}>", name, email),
            (Some(name), None) => println!("{} (email is empty)", name),
            (None, Some(email)) => println!("<{}> (name is empty)", email),
            (None, None) => println!("name and email are empty"),
        };
        Ok(())
    }

    fn set(
        name: &str,
        email: EmailAddress,
        config_file_location: Option<ConfigFileLocation>,
    ) -> Result<()> {
        let author = Author::new(Some(name), Some(email));
        let config_file_location = config_file_location.unwrap_or(ConfigFileLocation::Local);
        git::set_author(config_file_location, &author)?;
        println!("set {} author: {}", config_file_location, author);
        Ok(())
    }

    fn unset(config_file_location: Option<ConfigFileLocation>) -> Result<()> {
        git::unset_author(config_file_location)?;
        Ok(())
    }

    fn replace() -> Result<()> {
        Ok(())
    }
}
