use serde::*;
use jsonwebtoken::*;
use chrono::{Utc, TimeZone, Duration, DateTime};
use std::fs::File;
use std::io::Read;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use ini::Ini;

#[derive(Debug,Serialize,Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct Translation {
    pub translatedText: String,
    pub detectedSourceLanguage: Option<String>
}

#[derive(Debug,Serialize,Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct Detection {
    pub language: String,
    pub isReliable: bool,
    pub confidence: f32
}

pub struct TranslationContextKey;

#[derive(Debug,Clone)]
pub struct TranslationContext {
    key: KeyFile,
    pub token: String,
    pub token_expiry: DateTime<Utc>
}

pub enum TranslationError {
    KeyLoadError,
    KeyEncodingError,
    TokenCreationError,
    TokenRefreshError,
    TranslationError,
    DetectionError,
    ResponseError,
}

pub async fn initialize_translation(context: &Context, settings: &Ini){
    match settings.general_section().get("google_token_file") {
        None => println!("Google token file is not set in settings.ini, translation module is disabled"),
        Some("none") => println!("Google token file is not set in settings.ini, translation module is disabled"),
        Some(file) => match create_context(file.to_string()).await {
            Err(_) => println!("Failed to get token!"),
            Ok(ctx) => {
                let mut data = context.data.write().await;
                data.insert::<TranslationContextKey>(ctx);
            }
        }
    };
}

pub async fn translate_text(ctx: &TranslationContext, text: Vec<String>, target: Option<String>, source: Option<String>) -> std::result::Result<Vec<Translation>,TranslationError> {
    let request = RequestData {
        q: text,
        source: source,
        target: match target { None => Some("en".to_string()), Some(lang) => Some(lang)},
        format: Some("text".to_string())
    };

    let client = reqwest::Client::new();
    let resp: TranslationResultRaw = match client.post("https://translation.googleapis.com/language/translate/v2")
        .bearer_auth(&ctx.token)
        .json(&request)
        .send()
        .await {
        Err(why) => {
            println!("Failed translate query, error: {:?}",why);
            return Err(TranslationError::TranslationError);
        },
        Ok(content) => if content.status().is_success() {
            match content.json::<TranslationResultRaw>().await {
                Err(why) => {
                    println!("Failed translate query, error: {:?}",why);
                    return Err(TranslationError::TranslationError);
                },
                Ok(text) => text
            }
        } else {
            println!("Translation query did not return success, HTTP code: {:?}",content.status());
            return Err(TranslationError::ResponseError);
        }
    };

    return Ok(resp.data.translations);
}

pub async fn detect_text(ctx: &TranslationContext, text:    Vec<String>) -> std::result::Result<Vec<Detection>,TranslationError> {
    let request = RequestData {
        q: text,
        source: None,
        target: None,
        format: None
    };

    let client = reqwest::Client::new();
    let resp = match client.post("https://translation.googleapis.com/language/translate/v2/detect")
        .bearer_auth(&ctx.token)
        .json(&request)
        .send()
        .await {
        Err(why) => {
            println!("Failed detect query, error: {:?}",why);
            return Err(TranslationError::DetectionError);
        },
        Ok(content) => if content.status().is_success() {
            match content.json::<DetectionResultRaw>().await {
                Err(why) => {
                    println!("Failed detect query, error: {:?}",why);
                    return Err(TranslationError::DetectionError);
                },
                Ok(text) => text
            }
        } else {
            println!("Detection query did not return success, HTTP code: {:?}",content.status());
            return Err(TranslationError::ResponseError);
        }
    };
    let detected = resp.data.detections.iter().map(|d| d[0].clone()).collect();
    return Ok(detected);
}

pub async fn refresh_context(ctx: TranslationContext) -> Result<TranslationContext,TranslationError> {
    let mut header = Header::new(Algorithm::RS256);
    header.typ = Some("JWT".to_string());
    let enc_key = match EncodingKey::from_rsa_pem(ctx.key.private_key.as_bytes()) {
        Err(why) => {
            println!("Failed to create JWT encoding key: {:?}",why);
            return Err(TranslationError::KeyEncodingError);
        },
        Ok(key) => key
    };
    let expiry = Utc::now() + Duration::hours(1);
    let claims = Claims {
        iss: ctx.key.client_email.clone(),
        scope: "https://www.googleapis.com/auth/cloud-translation".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: expiry.timestamp(),
        iat: Utc::now().timestamp()
    };

    let jwt = match encode(&header, &claims, &enc_key) {
        Err(e) => {
            println!("Failed to encode JWT: {:?}",e);
            return Err(TranslationError::TokenRefreshError);
        },
        Ok(res) => res
    };

    let params = [("grant_type","urn:ietf:params:oauth:grant-type:jwt-bearer"),("assertion",&jwt)];
    let client = reqwest::Client::new();
    let resp = match client.post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await  {
        Err(why) => {
            println!("Failed to get token from auth service: {:?}",why);
            return Err(TranslationError::TokenRefreshError);
        },
        Ok(resp) => match resp.json::<TokenResult>().await {
            Err(why) => {
                println!("Failed to decode token result: {:?}",why);
                return Err(TranslationError::TokenRefreshError);
            },
            Ok(json) => json
        }
    };
    return Ok(TranslationContext { key: ctx.key, token: resp.access_token, token_expiry: expiry });
}

// ------------------- Internal Stuff -------------------

impl TypeMapKey for TranslationContextKey{
    type Value = TranslationContext;
}

#[derive(Debug,Serialize,Deserialize)]
struct Claims{
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64
}

#[derive(Debug,Serialize,Deserialize)]
struct RequestData {
    q: Vec<String>,
    source: Option<String>,
    target: Option<String>,
    format: Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
struct TokenResult {
    access_token: String
}

#[derive(Debug,Serialize,Deserialize)]
struct TranslationDataRaw {
    translations: Vec<Translation>
}

#[derive(Debug,Serialize,Deserialize)]
struct TranslationResultRaw {
    data: TranslationDataRaw
}

#[derive(Debug,Serialize,Deserialize)]
struct DetectionDataRaw {
    detections: Vec<Vec<Detection>>
}

#[derive(Debug,Serialize,Deserialize)]
struct DetectionResultRaw {
    data: DetectionDataRaw
}

#[derive(Debug,Serialize,Deserialize,Clone)]
struct KeyFile{
    r#type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String, 
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String
}

async fn create_context(key_file: String) -> std::result::Result<TranslationContext,TranslationError>{
    let mut file = File::open(key_file).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let key_file: KeyFile = match serde_json::from_str(&data){
        Err(why) => {
            println!("Failed to open key file, error: {:?}",why);
            return Err(TranslationError::KeyLoadError);
        },
        Ok(k) => k
    };

    match refresh_context(TranslationContext {key: key_file, token: "".to_string(), token_expiry: Utc.timestamp(0,0)}).await {
        Err(_) => {
            println!("Failed to create context!");
            return Err(TranslationError::TokenCreationError);
        },
        Ok(ctx) => return Ok(ctx)
    }
}