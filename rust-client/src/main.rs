mod consts;
mod structs;
mod transactions;

use crate::transactions::add_to_whitelist::add_to_whitelist;
use crate::transactions::claim::claim;
use crate::transactions::generate_vault::generate_vault;
use crate::transactions::stake::stake;
use crate::transactions::unstake::unstake;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("generate_vault_address")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add_to_whitelist")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("creator")
                        .short("c")
                        .long("creator")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("stake")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("nft")
                        .short("n")
                        .long("nft")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("unstake")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("nft")
                        .short("n")
                        .long("nft")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("claim")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("nft")
                        .short("n")
                        .long("nft")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("claim") {
        claim(matches);
    }

    if let Some(matches) = matches.subcommand_matches("unstake") {
        unstake(matches);
    }

    if let Some(matches) = matches.subcommand_matches("stake") {
        stake(matches);
    }

    if let Some(matches) = matches.subcommand_matches("add_to_whitelist") {
        add_to_whitelist(matches);
    }

    if let Some(matches) = matches.subcommand_matches("generate_vault_address") {
        generate_vault(matches);
    }
}
