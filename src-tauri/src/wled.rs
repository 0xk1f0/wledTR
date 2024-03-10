use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct NormalResponse {
    success: bool,
}

#[derive(Deserialize)]
struct PowerResponse {
    on: bool,
}

pub async fn send_power(host: String) -> Result<String, String> {
    // concat to full URL
    let url = format!("http://{}/json/state", host);

    // assemble body
    let payload = serde_json::json!({
        "on": "t",
        "v": true
    });

    // post to host
    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // convert to JSON
    let json_body = response
        .json::<PowerResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if json_body.on == true {
        return Ok("on".to_string());
    } else {
        return Ok("off".to_string());
    }
}

pub async fn send_color(host: String, rgb: (u8, u8, u8)) -> Result<String, String> {
    // concat to full URL
    let url = format!("http://{}/json/state", host);

    // assemble body
    let payload = serde_json::json!({
        "seg": [
            {
                "col": [
                    [rgb.0, rgb.1, rgb.2]
                ]
            }
        ]
    });

    // post to host
    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // convert to JSON
    let json_body = response
        .json::<NormalResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if json_body.success == true {
        return Ok("ok".to_string());
    }

    Err("Something went wrong".to_string())
}
