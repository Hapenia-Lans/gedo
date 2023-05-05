use clap::Parser;

/**
 * Copyright (c) 2023 hapenia
 *
 * This software is released under the MIT License.
 * https://opensource.org/licenses/MIT
 */
mod app;

fn main() -> anyhow::Result<()> {
    let appconfig = app::AppConfig::load_or_new()?;
    let manifest = app::Manifest::load_or_new()?;
    let package_list = app::PackageList::load_or_new()?;
    let app = app::App {
        appconfig,
        manifest,
        package_list,
    };
    let args = app::Args::parse();
    app.run(args)
}
