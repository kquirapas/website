use anyhow::Context;
use clap::{Arg, Command, crate_authors, crate_description, crate_name, crate_version};
use website_app::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            Command::new("app")
                .about("Run the app.")
                .arg(
                    Arg::new("PORT")
                        .short('p')
                        .long("port")
                        .help("Set the port to be used by the app server.")
                        .required(true),
                )
                .arg(
                    Arg::new("BASE_DIR")
                        .short('b')
                        .long("base-dir")
                        .help("Set the base directory where public assets are served from. Used for \\public.")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("app", app_matches)) => {
            let port = app_matches.get_one::<String>("PORT").unwrap(); // OK. Handled by clap.
            let base_dir = app_matches.get_one::<String>("BASE_DIR").unwrap(); // OK. Handled by clap.
            let app_config = Config::new(port, base_dir);
            println!("app_config: {app_config:?}");
            website_app::run(app_config).await?
        }
        _ => { /* Handled by clap-rs */ }
    }

    Ok(())
}
