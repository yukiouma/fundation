use reqwest::header::HeaderMap;

const ACCEPT: &str = "Accept";
const ACCEPT_ENCODING: &str = "Accept-Encoding";
const ACCEPT_LANGUAGE: &str = "Accept-Language";
const CONNECTION: &str = "Connection";
const COOKIE: &str = "Cookie";
const HOST: &str = "Host";
const REFERER: &str = "Referer";
const SEC_FETCH_DEST: &str = "Sec-Fetch-Dest";
const SEC_FETCH_MODE: &str = "Sec-Fetch-Mode";
const SEC_FETCH_SITE: &str = "Sec-Fetch-Site";
const USER_AGENT: &str = "User-Agent";
const SEC_CH_UA: &str = "sec-ch-ua";
const SEC_CH_UA_MOBILE: &str = "sec-ch-ua-mobile";
const SEC_CH_UA_PLATFORM: &str = "sec-ch-ua-platform";

pub fn header_map(cookie: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(ACCEPT, "*/*".parse().unwrap());
    h.insert(ACCEPT_ENCODING, "gzip, deflate, br, zstd".parse().unwrap());
    h.insert(
        ACCEPT_LANGUAGE,
        "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7,en-AS;q=0.6"
            .parse()
            .unwrap(),
    );
    h.insert(CONNECTION, "keep-alive".parse().unwrap());
    h.insert(COOKIE, cookie.parse().unwrap());
    h.insert(HOST, "api.fund.eastmoney.com".parse().unwrap());
    h.insert(REFERER, "https://fundf10.eastmoney.com/".parse().unwrap());
    h.insert(SEC_FETCH_DEST, "script".parse().unwrap());
    h.insert(SEC_FETCH_MODE, "no-cors".parse().unwrap());
    h.insert(SEC_FETCH_SITE, "same-site".parse().unwrap());
    h.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse().unwrap());
    h.insert(
        SEC_CH_UA,
        r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
            .parse()
            .unwrap(),
    );
    h.insert(SEC_CH_UA_MOBILE, "?0".parse().unwrap());
    h.insert(SEC_CH_UA_PLATFORM, r#""Windows""#.parse().unwrap());

    h
}
