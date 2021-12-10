//! An Aries Cloudagent Controller to interact with Aries instances for data manipulation
//! run `accf -e=XXX invite` to run the example script

#[macro_use]
extern crate clap;

use crate::agent::agent::HttpAgentExtended;
use crate::agent::http_agent::HttpAgent;
use clap::App;

mod agent;
mod cli;
mod error;
mod typing;
mod utils;

/// Initializes the application
#[tokio::main]
async fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml).get_matches();

    // Takes a path, but prepends the home directory... kinda sketchy
    let endpoint_from_config =
        utils::config::get_value("/.config/accf/ex.ini", "Default", "endpoint");

    // create an httpAgent when you supply an endpoint
    let agent = match matches.value_of("endpoint") {
        Some(endpoint) => HttpAgent::new(endpoint),
        None => match endpoint_from_config {
            Some(e) => HttpAgent::new(e.as_str()),
            None => error::throw(error::Error::NoSuppliedEndpoint),
        },
    };

    match agent.check_endpoint().await {
        Err(e) => error::throw(e),
        Ok(_) => true,
    };

    // Matches the `agent` subcommand
    if let Some(matches_agent) = matches.subcommand_matches("invite") {
        let auto_accept = matches_agent.is_present("auto-accept");
        let multi_use = matches_agent.is_present("multi-use");
        let alias = matches_agent.value_of("alias");
        let qr = matches_agent.is_present("qr");
        let toolbox = matches_agent.is_present("toolbox");

        let config = typing::InviteConfiguration {
            auto_accept,
            multi_use,
            alias,
            qr,
            toolbox,
        };

        // create agent and convert config
        cli::invite::run(agent, config).await
    }
}
