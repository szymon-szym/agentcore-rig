use aws_credential_types::{
    Credentials,
    provider::{ProvideCredentials, error::CredentialsError},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct MmdsCredentials {
    #[serde(rename = "AccessKeyId")]
    access_key_id: String,
    #[serde(rename = "SecretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "Token")]
    session_token: String,
    #[serde(rename = "Expiration")]
    expiration: String,
}

#[derive(Debug)]
pub struct MmdsProvider {
    client: reqwest::Client,
    endpoint: String,
}

impl MmdsProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: String::from("http://169.254.169.254"),
        }
    }
}

impl ProvideCredentials for MmdsProvider {
    fn provide_credentials<'a>(
        &'a self,
    ) -> aws_credential_types::provider::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        aws_credential_types::provider::future::ProvideCredentials::new(async move {
            // copy async body here
            let role_name_url = format!(
                "{}/latest/meta-data/iam/security-credentials/",
                self.endpoint
            );

            let role_name = self
                .client
                .get(&role_name_url)
                .send()
                .await
                .map_err(|e| CredentialsError::provider_error(Box::new(e)))?
                .text()
                .await
                .map_err(|e| CredentialsError::provider_error(Box::new(e)))?;

            let credentials_url = format!(
                "{}/latest/meta-data/iam/security-credentials/{}",
                self.endpoint, role_name
            );

            let credentials_raw = self
                .client
                .get(&credentials_url)
                .send()
                .await
                .map_err(|e| CredentialsError::provider_error(Box::new(e)))?;

            let credentials_json = credentials_raw
                .json::<serde_json::Value>()
                .await
                .map_err(|e| CredentialsError::provider_error(Box::new(e)))?;

            let credentials: MmdsCredentials = serde_json::from_value(credentials_json)
                .map_err(|e| CredentialsError::provider_error(Box::new(e)))?;

            let expiration_time = std::time::SystemTime::from(
                chrono::DateTime::parse_from_rfc3339(&credentials.expiration)
                    .map_err(|e| CredentialsError::provider_error(Box::new(e)))?,
            );

            Ok(Credentials::new(
                credentials.access_key_id,
                credentials.secret_access_key,
                Some(credentials.session_token),
                Some(expiration_time),
                "MmdsProvider",
            ))
        })
    }
}
