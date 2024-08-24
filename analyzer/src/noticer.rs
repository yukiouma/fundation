// use base64::{engine::general_purpose::URL_SAFE, Engine as _};
// use hmac::{Hmac, Mac};
// use reqwest::{header::CONTENT_TYPE, Client};
// use serde::Serialize;
// use sha2::Sha256;
// use urlencoding::encode;

// pub struct Noticer {
//     client: Client,
//     token: String,
//     secret: String,
// }

// #[derive(Debug, Serialize)]
// struct SendRequestBody {
//     pub text: String,
// }

// pub struct NoticerConstructor<'a> {
//     pub token: &'a str,
//     pub secret: &'a str,
// }

// impl Noticer {
//     pub fn new(param: &NoticerConstructor) -> Noticer {
//         let client = Client::new();
//         Noticer {
//             client,
//             token: param.token.into(),
//             secret: param.secret.into(),
//         }
//     }
//     pub async fn send(&self, content: &str) -> anyhow::Result<()> {
//         let timestamp = format!("{}", chrono::Local::now().timestamp_millis());
//         let sign = self.sign(&timestamp)?;
//         let url = bot_url(&self.token, &timestamp, &sign);
//         println!("{}", url);
//         let request_body = SendRequestBody {
//             text: content.into(),
//         };
//         let response = self
//             .client
//             .post(&url)
//             .header(CONTENT_TYPE, "application/json")
//             .body(serde_json::to_vec(&request_body)?)
//             .send()
//             .await?;
//         println!("{}", response.text().await?);
//         Ok(())
//     }
//     fn sign(&self, timestamp: &str) -> anyhow::Result<String> {
//         let string_to_sign = format!("{}\n{}", timestamp, self.secret);
//         let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())?;
//         mac.update(string_to_sign.as_bytes());
//         let sign = hex::encode(mac.finalize().into_bytes());
//         let sign = URL_SAFE.encode(sign.as_bytes());
//         let sign = encode(&sign).to_string();
//         Ok(sign)
//     }
// }

// fn bot_url(token: &str, timestamp: &str, sign: &str) -> String {
//     format!(
//         "https://oapi.dingtalk.com/robot/send?access_token={}&timestamp={}&sign={}",
//         token, timestamp, sign
//     )
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[tokio::test]
//     async fn notcier_test() {
//         let content = "叽叽叽叽";
//         let param = NoticerConstructor {
//             token: "300c7e6b17abe49757e5dc9f463eca96e025567521820825ed04fc8975fff96e",
//             secret: "SEC65f907bf84f13b4309e4fa3244848337c8d63dd0bd71a1a9ed326638de8f1589",
//         };
//         let noticer = Noticer::new(&param);
//         noticer.send(content).await.unwrap();
//         // println!("{}", noticer.sign().unwrap())
//     }
// }
