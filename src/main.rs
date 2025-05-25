#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{collections::HashMap, path::PathBuf, process::Command as StdCommand};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(not(target_os = "windows"))]
use anyhow::bail;

#[cfg(target_os = "windows")]
use registry::{Data, Hive, Security};

#[cfg(target_os = "windows")]
use elevated_command::Command;

#[cfg(target_os = "windows")]
use utfx::U16CString;

use clap::Parser;
use directories::{BaseDirs, UserDirs};
use serde::Deserialize;
use url::Url;
use which::which;

#[derive(Parser, Debug)]
struct Cli {
    url: Option<String>,
}

#[derive(Deserialize, Default)]
struct Config {
    default_vault: Option<String>,
    #[serde(default)]
    vaults: HashMap<String, PathBuf>,
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

fn get_dendron_dir(vault: &str) -> anyhow::Result<PathBuf> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("No base dirs found"))?;
    let user_dirs = UserDirs::new().ok_or_else(|| anyhow::anyhow!("No user dirs found"))?;

    let default_dendron_dir = user_dirs.home_dir().join("Dendron");

    let config_path = base_dirs.config_local_dir().join("dendron-handler.json");
    let config = if config_path.exists() {
        let config: Config = serde_json::from_str(&std::fs::read_to_string(config_path)?)?;
        config
    } else {
        Config::default()
    };

    if vault.is_empty() || vault == "default" {
        if let Some(default_vault) = config.default_vault {
            return Ok(config
                .vaults
                .get(&default_vault)
                .unwrap_or(&default_dendron_dir)
                .clone());
        }
        return Ok(default_dendron_dir);
    }

    Ok(config
        .vaults
        .get(vault)
        .ok_or(anyhow::anyhow!("Vault {} not found in config file", vault))?
        .clone())
}

#[cfg(not(target_os = "windows"))]
fn register_protocol_handler() -> anyhow::Result<()> {
    bail!("Automatic handler registration is only supported on Windows");
}

#[cfg(target_os = "windows")]
fn register_protocol_handler() -> anyhow::Result<()> {
    if !Command::is_elevated() {
        let cmd = StdCommand::new(std::env::current_exe()?);
        let elevated_cmd = Command::new(cmd);
        _ = elevated_cmd.output().unwrap();
        return Ok(());
    }

    let hive = Hive::ClassesRoot.create("dendron", Security::Write)?;
    hive.set_value("URL Protocol", &Data::String(U16CString::from_str("")?))?;
    let hive = Hive::ClassesRoot.create(r"dendron\shell\open\command", Security::Write)?;
    let command = format!("{} \"%1\"", std::env::current_exe()?.to_string_lossy());
    hive.set_value("", &Data::String(U16CString::from_str(command)?))?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Cli { url } = Cli::parse();
    if let Some(url) = url {
        let url = Url::parse(&url)?;
        let dendron_dir = PathBuf::from(
            shellexpand::full(
                get_dendron_dir(url.domain().unwrap_or_default())?
                    .to_string_lossy()
                    .as_ref(),
            )?
            .into_owned(),
        );
        let mut cmd = StdCommand::new(which("code")?);

        // start without window
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.arg(format!(
            "{}",
            dendron_dir.join("dendron.code-workspace").to_string_lossy()
        ));
        cmd.args([
            "--goto",
            &dendron_dir
                .join("notes")
                .join(url.path().trim_start_matches('/'))
                .to_string_lossy(),
        ]);
        cmd.spawn()?;
    } else {
        register_protocol_handler()?;
    }

    Ok(())
}
