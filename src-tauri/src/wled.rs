use crate::responses::{InfoResponse, NormalResponse, PowerResponse};
use reqwest::Client;
use std::time::Duration;

pub struct WLEDController;

impl WLEDController {
    pub async fn check_info(host: String) -> Result<String, String> {
        // concat to full URL
        let url = format!("http://{}/json/info", host);

        // post to host
        let response = Client::new()
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|_| "Request Error".to_string())?;

        // get response text
        let json_body = response
            .json::<InfoResponse>()
            .await
            .map_err(|_| "Invalid Device".to_string())?;

        if json_body.ip != "" {
            return Ok(json_body.ip);
        }

        Err("no ip".to_string())
    }

    pub async fn get_state(host: String) -> Result<String, String> {
        // concat to full URL
        let url = format!("http://{}/json/state", host);

        // post to host
        let response = Client::new()
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|_| "Request Error".to_string())?;

        // get response text
        let json_body = response
            .text()
            .await
            .map_err(|_| "Response Error".to_string())?;

        Ok(json_body)
    }

    pub async fn get_info(host: String) -> Result<String, String> {
        // concat to full URL
        let url = format!("http://{}/json/info", host);

        // post to host
        let response = Client::new()
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|_| "Request Error".to_string())?;

        // get response text
        let json_body = response
            .text()
            .await
            .map_err(|_| "Response Error".to_string())?;

        Ok(json_body)
    }

    pub async fn set_power(host: String) -> Result<String, String> {
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
            .timeout(Duration::from_secs(5))
            .json(&payload)
            .send()
            .await
            .map_err(|_| "Response Error".to_string())?;

        // convert to JSON
        let json_body = response
            .json::<PowerResponse>()
            .await
            .map_err(|_| "Decode Error".to_string())?;

        if json_body.on == true {
            return Ok("on".to_string());
        } else {
            return Ok("off".to_string());
        }
    }

    pub async fn set_brightness(host: String, brightness: u8) -> Result<String, String> {
        // concat to full URL
        let url = format!("http://{}/json/state", host);

        // assemble body
        let payload = serde_json::json!({
            "bri": brightness
        });

        // post to host
        let response = Client::new()
            .post(url)
            .timeout(Duration::from_secs(5))
            .json(&payload)
            .send()
            .await
            .map_err(|_| "Response Error".to_string())?;

        // convert to JSON
        let json_body = response
            .json::<NormalResponse>()
            .await
            .map_err(|_| "Decode Error".to_string())?;

        if json_body.success == true {
            return Ok("ok".to_string());
        }

        Err("Something went wrong".to_string())
    }

    pub async fn set_color(host: String, rgb: (u8, u8, u8)) -> Result<String, String> {
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
            .timeout(Duration::from_secs(5))
            .json(&payload)
            .send()
            .await
            .map_err(|_| "Request Error".to_string())?;

        // convert to JSON
        let json_body = response
            .json::<NormalResponse>()
            .await
            .map_err(|_| "Response Error".to_string())?;

        if json_body.success == true {
            return Ok("ok".to_string());
        }

        Err("Something went wrong".to_string())
    }
}
