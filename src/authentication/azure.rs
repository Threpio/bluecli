use oauth2::{
    AuthUrl, ClientId, DeviceAuthorizationResponse, DeviceAuthorizationUrl,
    EmptyExtraDeviceAuthorizationFields, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest::Client;
use tokio::time::sleep;

pub async fn code_auth() -> anyhow::Result<()> {
    // Use Microsoft's public Azure CLI client ID
    let azure_cli_client_id = "04b07795-8ddb-461a-bbee-02f9e1bf7b46";

    let client = BasicClient::new(ClientId::new(azure_cli_client_id.to_string()))
        .set_device_authorization_url(DeviceAuthorizationUrl::new(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode".to_string(),
        )?)
        .set_auth_uri(AuthUrl::new(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        )?);

    // Start the device code flow
    let details: DeviceAuthorizationResponse<EmptyExtraDeviceAuthorizationFields> = client
        .exchange_device_code()
        .add_scope(Scope::new(
            "https://management.azure.com/.default".to_string(),
        ))
        .request_async(&Client::new())
        .await?;

    // Prompt user to authenticate
    println!("\n🔑 Please authenticate:");
    println!("Visit:    {}", details.verification_uri().as_str());
    println!("Enter code: {}", details.user_code().secret().as_str());

    // Add polling logic to wait for user authentication
    let token = client
        .exchange_device_access_token(&details)
        .request_async(&Client::new(), sleep, None)
        .await?;

    println!("Access token: {}", token.access_token().secret());

    //println!("\n✅ Access token received!");
    //println!("Access token: {}", token.access_token().secret());

    Ok(())
}
