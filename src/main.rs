use minify_html::{minify, Cfg};
use scraper::{Html, Selector};
use std::fs::{read, write};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct HtmlProcessorError(String);

impl fmt::Display for HtmlProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HtmlProcessorError: {}", self.0)
    }
}

impl Error for HtmlProcessorError {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// 压缩 HTML 文件
pub fn minify_html(input_path: &str) -> Result<PathBuf> {
    let input_path_obj = Path::new(input_path);
    let file_stem = input_path_obj.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| HtmlProcessorError("Invalid input file name".into()))?;
    
    let output_path = input_path_obj
        .parent()
        .map(|p| p.join(format!("{}.min.html", file_stem)))
        .unwrap_or_else(|| PathBuf::from(format!("{}.min.html", file_stem)));

    let code = read(input_path)?;

    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.keep_comments = false;
    cfg.keep_input_type_text_attr = true;
    cfg.keep_ssi_comments = false;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.remove_bangs = false;
    cfg.remove_processing_instructions = false;

    let minified = minify(&code, &cfg);
    write(&output_path, minified)?;

    Ok(output_path)
}

/// 获取外部资源内容
fn fetch_external_resource(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;
    
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        eprintln!("Warning: Failed to fetch {}: {}", url, resp.status());
        return Err(Box::new(HtmlProcessorError(format!("Failed to fetch {}", url))));
    }
    Ok(resp.text()?)
}

/// 内联所有外部资源
pub fn all_in_one(input_path: &str) -> Result<PathBuf> {
    let input_path_obj = Path::new(input_path);
    let file_stem = input_path_obj.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| HtmlProcessorError("Invalid input file name".into()))?;
    
    let output_path = input_path_obj
        .parent()
        .map(|p| p.join(format!("{}.allinone.html", file_stem)))
        .unwrap_or_else(|| PathBuf::from(format!("{}.allinone.html", file_stem)));

    let html_content = read(input_path)?;
    let html_str = String::from_utf8_lossy(&html_content);
    let document = Html::parse_document(&html_str);

    // 保留原始 HTML 结构，只替换特定标签内容
    let mut processed_html = html_str.to_string();

    // 处理 script 标签
    let script_selector = Selector::parse("script[src]").unwrap();
    for script in document.select(&script_selector) {
        if let Some(src) = script.value().attr("src") {
            if src.starts_with("http") {
                match fetch_external_resource(src) {
                    Ok(content) => {
                        let original = script.html();
                        let replacement = format!("<script type=\"{}\"{}>{}</script>",
                            script.value().attr("type").unwrap_or(""),
                            script.value().attrs()
                                .filter(|(name, _)| *name != "src" && *name != "type")
                                .map(|(name, value)| format!(" {}=\"{}\"", name, value))
                                .collect::<String>(),
                            content);
                        processed_html = processed_html.replace(&original, &replacement);
                    },
                    Err(e) => {
                        eprintln!("Warning: Could not inline {}: {}", src, e);
                    }
                }
            }
        }
    }

    // 处理 link 标签
    let link_selector = Selector::parse("link[rel=stylesheet]").unwrap();
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            if href.starts_with("http") {
                match fetch_external_resource(href) {
                    Ok(content) => {
                        let original = link.html();
                        let replacement = format!("<style type=\"text/css\"{}>{}</style>",
                            link.value().attrs()
                                .filter(|(name, _)| *name != "href" && *name != "rel")
                                .map(|(name, value)| format!(" {}=\"{}\"", name, value))
                                .collect::<String>(),
                            content);
                        processed_html = processed_html.replace(&original, &replacement);
                    },
                    Err(e) => {
                        eprintln!("Warning: Could not inline {}: {}", href, e);
                    }
                }
            }
        }
    }

    write(&output_path, processed_html)?;
    Ok(output_path)
}

fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <path_to_html_file>", std::env::args().next().unwrap());
        return;
    }

    let input_path = std::env::args().nth(1).expect("Expected a path to an HTML file");

    // 1. 生成 min.html
    match minify_html(&input_path) {
        Ok(minified_path) => {
            println!("Minified HTML written to {}", minified_path.display());
        },
        Err(e) => eprintln!("Error minifying HTML: {}", e),
    }

    // 2. 生成 allinone.html
    match all_in_one(&input_path) {
        Ok(all_in_one_path) => {
            println!("All-in-one HTML written to {}", all_in_one_path.display());
            
            // 3. 生成 allinone.min.html
            match minify_html(all_in_one_path.to_str().unwrap()) {
                Ok(minified_path) => {
                    println!("Minified all-in-one HTML written to {}", minified_path.display());
                },
                Err(e) => eprintln!("Error minifying all-in-one HTML: {}", e),
            }
        },
        Err(e) => eprintln!("Error creating all-in-one HTML: {}", e),
    }
}