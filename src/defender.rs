use std::{process::Command, fmt::format};
use powershell_script;
use crate::{globals::*, unwrap::CustomUnwrap};
use anyhow::*;

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
    let lines: Vec<&str> = output.lines().collect();
    for line in lines.iter(){
        if line.starts_with("SubmitSamplesConsent") && !line.ends_with("2"){
            println!("Executing: {}", SUBMITSAMPLESCONSENT);
            powershell_script::run(SUBMITSAMPLESCONSENT).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("MAPSReporting") && !line.ends_with("0") {
            println!("Executing: {}", MAPSREPORTING);
            powershell_script::run(MAPSREPORTING).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("HighThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", HIGHTHREATDEFAULTACTION);
            powershell_script::run(HIGHTHREATDEFAULTACTION).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("ModerateThreatDefaultAction") && !line.ends_with("6"){
            println!("Executing: {}", MODERATETHREATDEFAULTACTION);
            powershell_script::run(MODERATETHREATDEFAULTACTION).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("LowThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", LOWTHREATDEFAULTACTION);
            powershell_script::run(LOWTHREATDEFAULTACTION).expect_log(&format!("Error setting {}", line));
        } else if line.starts_with("SevereThreatDefaultAction") && !line.ends_with("6") {
            println!("Executing: {}", SEVERETHREATDEFAULTACTION);
            powershell_script::run(SEVERETHREATDEFAULTACTION).expect_log(&format!("Error setting {}", line));
        }
        else if line.starts_with("ExclusionExtension") {}
        else if line.starts_with("ExclusionIpAddress") {}
        else if line.starts_with("ExclusionPath") {}
        else if line.starts_with("EXclusionProcess") {}
        else {
            let true_line: Vec<&str> = line.split(" ").collect();
            let action = format!("Set-MpPreference -{} $true", true_line.first().context("")?);
            println!("Executing: {}", action);
            powershell_script::run(&action).expect_log(&format!("Error setting {}", line));
        }
    }
    Ok(())
}