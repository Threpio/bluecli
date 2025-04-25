use azure_core::credentials::AccessToken;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Subscriptions {
    value: Vec<Subscription>,
}

#[derive(Deserialize, Debug)]
struct Subscription {
    subscriptionId: String,
}

// get_subscriptions function takes an AccessToken as an argument
// It generates the URL and parses the response in a very crude manner
pub(crate) async fn get_subscriptions(access_token: AccessToken) {

    let url = url::Url::parse("https://management.azure.com/subscriptions?api-version=2022-12-01")
        .expect("url parse");

    let response = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.token.secret()),
        )
        .send()
        .await
        .expect("send access token")
        .text()
        .await
        .expect("text");

    let subs: Subscriptions = serde_json::from_str(&response)
        .expect("deserialize subscriptions failure");
    println!("Subscriptions:");
    for sub in subs.value {
        println!("{}", sub.subscriptionId);
    }
}