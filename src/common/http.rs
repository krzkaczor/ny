use eyre::Result;

use mockall::automock;

#[automock]
pub trait HttpClient {
    fn request_if_success(&self, url: &str) -> Result<bool>;
}

pub struct RealHttpClient {}
impl HttpClient for RealHttpClient {
    fn request_if_success(&self, url: &str) -> Result<bool> {
        let resp = reqwest::blocking::get(url)?;
        Ok(resp.status().is_success())
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use mockall::predicate::*;

    pub fn expect_package_exist_in_registry(
        mock_http_client: &mut MockHttpClient,
        package: &str,
        success: bool,
    ) {
        let url = format!("https://registry.npmjs.org/{}", package);
        mock_http_client
            .expect_request_if_success()
            .with(eq(url))
            .returning(move |_| Ok(success));
    }
}
