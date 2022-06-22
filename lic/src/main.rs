// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::{AppSettings, Parser, Subcommand};
use cotowali_error::Error;
use cotowali_source::Source;

#[derive(Parser)]
#[clap(
    name = "cotowali",
    version,
    about = "Cotowali compiler",
    help_template = "\
{name} - {about}

{usage-heading}
    {usage}
{all-args}"
)]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
struct App {
    #[clap(value_parser)]
    file: Option<PathBuf>,
    #[clap(subcommand)]
    command: Option<AppCommands>,
}

#[derive(Subcommand, Debug)]
enum AppCommands {
    Run {
        #[clap(value_parser)]
        file: Option<PathBuf>,
    },
}

fn read_source(input: Option<PathBuf>) -> Source {
    fn inner(input: Option<PathBuf>) -> io::Result<Source> {
        match input {
            Some(input) => Source::read_file(input),
            None => {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf)?;
                Ok(Source::dummy("stdin", buf))
            }
        }
    }
    match inner(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read file: {}", e);
            std::process::exit(1);
        }
    }
}

fn failed_to_compile(errs: Vec<Error>) -> ! {
    eprintln!(
        "{}",
        errs.iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join("\n")
    );
    std::process::exit(1);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let app = App::parse();
    match app.command {
        Some(AppCommands::Run { file }) => match cotowali::run(read_source(file)) {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                std::process::exit(output.status.code().unwrap());
            }
            Err(e) => failed_to_compile(e),
        },
        None => match cotowali::compile(read_source(app.file)) {
            Ok(output) => io::stdout().write_all(output.as_bytes()).unwrap(),
            Err(e) => failed_to_compile(e),
        },
    };
}
