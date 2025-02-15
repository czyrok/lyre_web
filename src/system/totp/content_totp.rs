use std::time::SystemTimeError;

use totp_rs::TOTP;

#[derive(Clone, Debug)]
pub struct ContentTotp {
    totp: TOTP,
}

impl ContentTotp {
    pub fn new(content_totp_uri: &str) -> Result<Self, totp_rs::TotpUrlError> {
        let totp = TOTP::from_url(content_totp_uri)?;

        Ok(Self { totp })
    }

    pub fn check(&self, token: &str) -> Result<bool, SystemTimeError> {
        self.totp.check_current(token)
    }
}
