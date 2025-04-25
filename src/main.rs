mod authentication;
mod subscriptions;

use azure_core::credentials::TokenCredential;
use azure_identity::DefaultAzureCredential;
use clap::Command;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let matches = Command::new("bluecli")
        .about("opinionated pre-prod azure cli")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("login")
                .long_flag("login")
                .about("trigger the oath login flow"),
        )
        .subcommand(
            Command::new("codelogin")
                .long_flag("codelogin")
                .about("trigger the code login flow"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("login", _)) => {
            let _subscription_id =
                std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

            let credential = DefaultAzureCredential::new()
                .map(Arc::new)
                .expect("DefaultAzureCredential::new");

            let access_token = credential
                .get_token(&["https://management.azure.com/.default"])
                .await
                .expect("get token");

            subscriptions::get_subscriptions(access_token.clone()).await;
        }
        Some(("codelogin", _)) => {
            authentication::azure::code_auth().await.expect("auth");
        }
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {values}...");
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if sync_matches.get_flag("info") {
                println!("Retrieving info for {values}...");
            } else {
                println!("Installing {values}...");
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {comma_sep}...");
            } else if let Some(queries) = query_matches.get_many::<String>("search") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {comma_sep}...");
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
