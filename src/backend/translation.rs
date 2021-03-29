use serde::*;
use jsonwebtoken::*;
use chrono::Utc;
use chrono::Duration;
use chrono::DateTime;
use std::fs::File;
use std::io::Read;

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
    q: String,
    source: Option<String>,
    target: String,
    format: String
}

#[derive(Debug,Serialize,Deserialize)]
struct TokenResult {
    access_token: String
}

#[derive(Debug,Serialize,Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct Translation {
    pub translatedText: String,
    pub detectedSourceLanguage: String
}

#[derive(Debug,Serialize,Deserialize)]
struct DataRaw {
    translations: Vec<Translation>
}

#[derive(Debug,Serialize,Deserialize)]
struct TranslationResultRaw {
    data: DataRaw
}

#[derive(Debug,Serialize,Deserialize)]
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

pub struct Token {
    pub token: String,
    pub valid_until: DateTime<Utc>
}

pub enum TranslationError {
    KeyLoadError,
    KeyEncodingError,
    TokenCreationError,
    TranslationError,
}

pub async fn create_token() -> std::result::Result<Token,TranslationError>{
    let mut file = File::open("BeanBot-bf935a27b851.json").unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let key_file: KeyFile = match serde_json::from_str(&data){
        Err(why) => {
            println!("Failed to open key file, error: {:?}",why);
            return Err(TranslationError::KeyLoadError);
        },
        Ok(k) => k
    };

    let mut header = Header::new(Algorithm::RS256);
    header.typ = Some("JWT".to_string());
    let enc_key = match EncodingKey::from_rsa_pem(key_file.private_key.as_bytes()) {
        Err(why) => {
            println!("Failed to create JWT encoding key: {:?}",why);
            return Err(TranslationError::KeyEncodingError);
        },
        Ok(key) => key
    };
    let expiry = Utc::now() + Duration::hours(1);
    let claims = Claims {
        iss: key_file.client_email,
        scope: "https://www.googleapis.com/auth/cloud-translation".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: expiry.timestamp(),
        iat: Utc::now().timestamp()
    };

    let jwt = match encode(&header, &claims, &enc_key) {
        Err(e) => {
            println!("Failed to encode JWT: {:?}",e);
            return Err(TranslationError::TokenCreationError);
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
            println!("Error why: {:?}",why);
            return Err(TranslationError::TokenCreationError);
        },
        Ok(resp) => match resp.json::<TokenResult>().await {
            Err(why) => {
                println!("Error why: {:?}",why);
                return Err(TranslationError::TokenCreationError);
            },
            Ok(json) => json
        }
    };
    return Ok(Token {token: resp.access_token, valid_until: expiry});
}

pub async fn translate(text: String, token: Token, target: Option<String>, source: Option<String>) -> std::result::Result<Translation,TranslationError> {
    let request = RequestData {
        q: text,
        source: source,
        target: match target { None => "en".to_string(), Some(lang) => lang},
        format: "text".to_string()
    };

    let client = reqwest::Client::new();
    let resp = match client.post("https://translation.googleapis.com/language/translate/v2")
        .bearer_auth(token.token)
        .json(&request)
        .send()
        .await {
        Err(why) => {
            println!("Failed translate query, error: {:?}",why);
            return Err(TranslationError::TranslationError);
        },
        Ok(content) => match content.json::<TranslationResultRaw>().await {
            Err(why) => {
                println!("Failed translate query, error: {:?}",why);
                return Err(TranslationError::TranslationError);
            },
            Ok(text) => text
        }
    };

    return Ok(resp.data.translations[0].clone());
}