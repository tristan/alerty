use std::{
    env,
    path::{self, PathBuf},
};

use alerty::{
    config::{Config, OutputType},
    format_current_datetime,
};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

fn main() {
    let test_mode = matches!(env::args().next().as_deref(), Some("test"));
    // check if there's a config file in the current path
    let config_file = path::Path::new("alerty.toml");
    let config_file = if config_file.exists() {
        config_file.to_owned()
    } else {
        // otherwise check $HOME/.config/alerty/config.toml
        let mut home_dir: PathBuf = env::var_os("HOME").map(PathBuf::from).unwrap();
        home_dir.push(".config");
        home_dir.push("alerty");
        home_dir.push("config.toml");
        home_dir
    };
    let config = Config::open(config_file).unwrap();
    let results = alerty::run(&config, test_mode).unwrap();

    if let Some(results) = results {
        if let Some(outputs) = &config.outputs {
            for output in outputs {
                match output {
                    OutputType::StdOut => {
                        println!("{results}")
                    }
                    OutputType::Email => {
                        // safe to unwrap as it should have been validated
                        let smtp = config.smtp.as_ref().unwrap();
                        let email = Message::builder()
                            .from(smtp.from.parse().unwrap())
                            .reply_to(smtp.to.parse().unwrap())
                            .to(smtp.to.parse().unwrap())
                            .subject(format_current_datetime(&smtp.subject))
                            .header(ContentType::TEXT_HTML)
                            .body(results.clone())
                            .unwrap();

                        let creds = Credentials::new(smtp.username.clone(), smtp.password.clone());

                        // Open a remote connection to gmail
                        let mailer = SmtpTransport::relay(&smtp.relay)
                            .unwrap()
                            .credentials(creds)
                            .build();
                        if let Err(e) = mailer.send(&email) {
                            eprintln!("Could not send email: {e:?}");
                        }
                    }
                    OutputType::File(path) => {
                        if let Err(e) = std::fs::write(path, &results) {
                            eprintln!("Could not write to file `{path:?}`: {e:?}");
                        } else {
                            eprintln!("Wrote: {path:?}");
                        }
                    }
                }
            }
        } else {
            println!("{results}");
        }
    } else {
        eprintln!("No results");
    }
}
