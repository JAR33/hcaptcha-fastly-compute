use fastly::http::StatusCode;
use fastly::{Dictionary, Error, Request, Response};
use glob::Pattern;

const DICT: &str = "hcaptcha";
const LOGS: &str = "hcaptcha";
const ORIGIN: &str = "Origin";
const CAPTCHA_BACKEND: &str = "hCaptcha";
const SITE_VERIFY_URL: &str = "https://hcaptcha.com/siteverify";
const HCAPTCHARESPONSE_HEADER: &str = "X-hCaptcha-Response";
const HCAPTCHA_SCORE_HEADER: &str = "X-hCaptcha-Score";
const HCAPTCHA_SCORE_REASON_HEADER: &str = "X-hCaptcha-Score-Reason";

#[derive(Default)]
pub struct Configuration {
    pub protected_paths: Vec<glob::Pattern>,
    pub method: String,
    pub sitekey: String,
    pub secret_key: String,
}

fn load_config() -> Option<Configuration> {
    let dict = Dictionary::open(DICT);
    let mut conf: Configuration = { Default::default() };

    conf.method = match dict.get("method") {
        Some(method) => method.to_uppercase(),
        _ => "GET".to_string(),
    };

    conf.sitekey = match dict.get("sitekey") {
        Some(sitekey) => sitekey,
        _ => return None,
    };

    conf.secret_key = match dict.get("secret_key") {
        Some(secret_key) => secret_key,
        _ => return None,
    };

    conf.protected_paths = match dict.get("protected_paths") {
        Some(protected_paths) => protected_paths
            .split(",")
            .filter(|&s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .filter_map(|s| match Pattern::new(&s) {
                Ok(p) => Some(p),
                _ => None,
            })
            .collect(),
        _ => return None,
    };

    Some(conf)
}

fn verify_request(req: &mut Request, conf: &Configuration) -> bool {
    let hcaptcharesponse = match req.get_header_str(HCAPTCHARESPONSE_HEADER) {
        Some(val) => val,
        _ => {
            log::info!("Header not found: {HCAPTCHARESPONSE_HEADER}");
            return false;
        }
    };

    let client_ip = match req.get_client_ip_addr() {
        Some(val) => val.to_string(),
        _ => {
            log::info!("Could not get client IP address");
            return false;
        }
    };

    let body = format!(
        "response={}&secret={}&sitekey={}&remoteip={}",
        hcaptcharesponse, conf.secret_key, conf.sitekey, client_ip
    );

    log::debug!(
        "Verifying {client_ip}, path: {} . Body: {body}",
        req.get_path()
    );

    let captcha_req = Request::post(SITE_VERIFY_URL)
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(body);

    let mut resp = match captcha_req.send(CAPTCHA_BACKEND) {
        Ok(rest) => rest,
        Err(e) => {
            log::info!("Failed to verify request: {e}");
            return false;
        }
    };

    let body: serde_json::Value = match resp.take_body_json::<serde_json::Value>() {
        Ok(val) => val,
        _ => {
            log::error!("Failed to parse hCaptcha body");
            return false;
        }
    };

    log::debug!(
        "hCaptcha response: {}",
        serde_json::to_string(&body).unwrap()
    );

    if body["success"] == false {
        log::info!("hCaptcha: success = false");
        return false;
    }

    match body["score"].as_i64() {
        Some(val) => req.set_header(HCAPTCHA_SCORE_HEADER, val.to_string()),
        _ => {}
    }

    match body["score_reason"].as_array() {
        Some(val) => req.set_header(
            HCAPTCHA_SCORE_REASON_HEADER,
            val.into_iter()
                .map(|v| v.as_str().unwrap().to_string() + " ")
                .collect::<String>(),
        ),
        _ => {}
    }

    return true;
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    log_fastly::init_simple(LOGS, log::LevelFilter::Debug);

    let conf = match load_config() {
        Some(conf) => conf,
        _ => {
            log::error!("Failed to retrieve configuration from the Dictionary");
            return Ok(Response::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    if !req.get_method().as_str().eq(&conf.method) {
        log::debug!(
            "Passing: not interested in method: {}",
            req.get_method().as_str()
        );
        return Ok(req.send(ORIGIN)?);
    };

    for protected_path in &conf.protected_paths {
        if protected_path.matches(req.get_path()) {
            match verify_request(&mut req, &conf) {
                true => {
                    log::info!("Request verified successfully");
                    return Ok(req.send(ORIGIN)?);
                }
                false => {
                    log::info!("Failed to verify request");
                    return Ok(Response::from_status(StatusCode::UNAUTHORIZED));
                }
            }
        }
    }

    log::debug!(
        "Passing: path is not in the protected list: {}",
        req.get_path()
    );
    Ok(req.send(ORIGIN)?)
}
