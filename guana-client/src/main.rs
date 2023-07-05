use std::{eprintln, print, println, process::ExitCode};

use guana_client::{Decision, GuanaClient, Result};
use owo_colors::OwoColorize;

use argh::FromArgs;
use rustyline::{error::ReadlineError, DefaultEditor};

#[derive(FromArgs, Debug)]
/// A client to a gRPC server that filters User-Agent strings.
struct Args {
    /// the server address to connect to
    #[argh(option)]
    pub address: Option<String>,

    /// user agents to check on the gRPC server. If none, interactive mode will be enabled.
    #[argh(positional)]
    pub user_agents: Vec<String>,
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        eprintln!("{}: {err}", "Error".red());

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run() -> Result {
    let args: Args = argh::from_env();
    let is_interactive_mode = args.user_agents.is_empty();

    let client = initialize_client_from_args(args).await?;

    if is_interactive_mode {
        // Read lines from the user and check them in the gRPC server for
        // as long as the user wants to
        interactive_loop(client).await?;
    }

    Ok(())
}

/// Given a single User-Agent string, check it in the gRPC server
/// and print it's output
async fn check_and_print_decision(
    client: &mut GuanaClient,
    user_agent: String,
) -> Result {
    print!("> User-Agent {}: ", user_agent.bold());

    let decision = client.check_user_agent(user_agent).await?;

    if decision == Decision::Allow {
        println!("{}", decision.as_str_name().green());
    } else {
        println!("{}", decision.as_str_name().red());
    }

    Ok(())
}

async fn initialize_client_from_args(
    args: Args,
) -> Result<GuanaClient> {
    let mut client = {
        let address = args.address.unwrap_or_else(|| {
            eprintln!(
                "> {} Address not set. Connecting to '[::1]:50051'",
                "Warn".yellow()
            );
            "http://[::1]:50051".into()
        });

        GuanaClient::connect(address).await?
    };

    // If there were any CLI-supplied user-agents, check them
    for user_agent in args.user_agents {
        let decision =
            client.check_user_agent(user_agent).await?;
        println!("{}", decision.as_str_name());
    }

    Ok(client)
}

async fn interactive_loop(mut client: GuanaClient) -> Result {
    // Note: this should never fail
    let mut editor = DefaultEditor::new()
        .expect("Failed to initialize Readline editor");

    loop {
        match editor.readline(">> ") {
            Ok(user_agent) => {
                check_and_print_decision(
                    &mut client,
                    user_agent,
                )
                .await?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Received Ctrl+C. Exiting");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Received EOF. Exiting");
                break;
            }
            Err(err) => {
                eprintln!("{}: {err}", "Error".red());
                break;
            }
        }
    }

    Ok(())
}
