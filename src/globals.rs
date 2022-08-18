pub const EXCLUSIONFOLDERCOMMAND: &str =
    r#"Add-MpPreference -ExclusionPath"#;
pub const EXCLUSIONEXTENTIONCOMMAND: &str =
    r#"Add-MpPreference -ExclusionExtension"#;
pub const EXCLUSIONPROCESSCOMMAND: &str =
    r#"Add-MpPreference -ExclusionProcess"#;
pub const EXCLUSIONIPADDRESSCOMMAND: &str =
    r#"Add-MpPreference -ExclusionIpAddress"#;
pub const SUBMITSAMPLESCONSENT: &str =
    r#"Set-MpPreference -SubmitSamplesConsent 2"#;
pub const MAPSREPORTING: &str =
    r#"Set-MpPreference -MAPSReporting 0"#;
pub const HIGHTHREATDEFAULTACTION: &str =
    r#"Set-MpPreference -HighThreatDefaultAction 6 -Force"#;
pub const MODERATETHREATDEFAULTACTION: &str =
    r#"Set-MpPreference -ModerateThreatDefaultAction 6"#;
pub const LOWTHREATDEFAULTACTION: &str =
    r#"Set-MpPreference -LowThreatDefaultAction 6"#;
pub const SEVERETHREATDEFAULTACTION: &str =
    r#"Set-MpPreference -SevereThreatDefaultAction 6"#;