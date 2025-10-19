use std::error::Error;

/// 打开浏览器
pub fn invoke_browser(url: &str, text: Option<&str>, rule: Option<&str>) -> Result<(), Box<dyn Error>> {
    let full_url = match text {
        Some(t) => {
            let rule_str = rule.unwrap_or("");
            format!("{}/{}{}", url, rule_str, t)
        },
        None => url.to_string(),
    };
    webbrowser::open(&full_url)?;
    Ok(())
}