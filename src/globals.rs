use wpkg_macro::encode;

pub const EXCLUSIONFOLDERCOMMAND: &str =
    &encode!(r#"Add-MpPreference -ExclusionPath"#);
pub const EXCLUSIONEXTENTIONCOMMAND: &str =
    &encode!(r#"Add-MpPreference -ExclusionExtension"#);
pub const EXCLUSIONPROCESSCOMMAND: &str =
    &encode!(r#"Add-MpPreference -ExclusionProcess"#);
pub const EXCLUSIONIPADDRESSCOMMAND: &str =
    &encode!(r#"Add-MpPreference -ExclusionIpAddress"#);
pub const SUBMITSAMPLESCONSENT: &str =
    &encode!(r#"Set-MpPreference -SubmitSamplesConsent 2"#);
pub const MAPSREPORTING: &str =
    &encode!(r#"Set-MpPreference -MAPSReporting 0"#);
pub const HIGHTHREATDEFAULTACTION: &str =
    &encode!(r#"Set-MpPreference -HighThreatDefaultAction 6 -Force"#);
pub const MODERATETHREATDEFAULTACTION: &str =
    &encode!(r#"Set-MpPreference -ModerateThreatDefaultAction 6"#);
pub const LOWTHREATDEFAULTACTION: &str =
    &encode!(r#"Set-MpPreference -LowThreatDefaultAction 6"#);
pub const SEVERETHREATDEFAULTACTION: &str =
    &encode!(r#"Set-MpPreference -SevereThreatDefaultAction 6"#);