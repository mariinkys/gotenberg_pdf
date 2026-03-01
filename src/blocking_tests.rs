use super::*;
use crate::BlockingClient as Client;
use std::time::Duration;

const HTML_CONTENT: &str = r#"
<!doctype html>
<html>
    <head><title>My PDF</title></head>
    <body><h1>Hello, PDF!</h1></body>
</html>
"#;

const DOCX_CONTENT: &[u8] = include_bytes!("../test_files/example.docx");

const PASSWORD_PROTECTED_ODT_CONTENT: &[u8] =
    include_bytes!("../test_files/example_password_protected.odt");

const BASE_URL: &str = "http://localhost:3000";

#[test]
fn test_url_to_pdf() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        skip_network_idle_events: Some(false),
        ..Default::default()
    };

    // Call the API and handle the result
    match client.pdf_from_url("https://example.com", options) {
        Ok(bytes) => {
            // Verify the response content
            assert!(!bytes.is_empty(), "PDF content should not be empty");
            println!("Received PDF content: {} bytes", bytes.len());

            // Save to local temp directory
            let temp_dir = std::env::temp_dir();
            let pdf_path = temp_dir.join("ocudigital.pdf");
            std::fs::write(&pdf_path, bytes).unwrap();
            println!("PDF saved to: {:?}", pdf_path);
        }
        Err(err) => {
            panic!("API call failed: {:?}", err);
        }
    }
}

#[test]
fn test_web_options_trace_id() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        trace_id: Some("test-trace-id".to_string()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_single_page() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        single_page: Some(true),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_paper_size() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        paper_width: Some("210mm".parse().unwrap()),
        paper_height: Some("297mm".parse().unwrap()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_margins() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        margin_top: Some("1in".parse().unwrap()),
        margin_bottom: Some("1in".parse().unwrap()),
        margin_left: Some("0.5in".parse().unwrap()),
        margin_right: Some("0.5in".parse().unwrap()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_prefer_css_page_size() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        prefer_css_page_size: Some(true),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_print_background() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        print_background: Some(true),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_landscape() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        landscape: Some(true),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_scale() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        scale: Some(1.5),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_page_ranges() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        native_page_ranges: Some("1-3,5".parse().unwrap()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_header_footer() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        header_html: Some("<h1>Header Test: <div class='title'></div></h1>".into()),
        footer_html: Some("Page Number: <div class='pageNumber'></div>".into()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_wait_delay() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        wait_delay: Some(Duration::from_secs(1)),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_emulated_media_type() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        emulated_media_type: Some("screen".parse().unwrap()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_fail_on_http_status_codes() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        fail_on_http_status_codes: Some(vec![404, 500]),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_metadata() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        metadata: Some(HashMap::from([
            ("Title".to_string(), "Test Document".into()),
            ("Author".to_string(), "Test Author".into()),
        ])),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_user_agent() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        user_agent: Some("TestUserAgent/1.0".into()),
        ..Default::default()
    };

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_web_options_negative_scale() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        // Negative scale should fail
        scale: Some(-1.0),
        ..Default::default()
    };

    let result = client.pdf_from_html(HTML_CONTENT, options);
    assert!(result.is_err(), "Expected negative scale to fail");
}

#[test]
fn test_web_options_unsupported_user_agent() {
    let client = Client::new(BASE_URL);

    let options = WebOptions {
        // Unsupported user agent format
        user_agent: Some("\0invalid_user_agent".into()),
        ..Default::default()
    };

    let result = client.pdf_from_html(HTML_CONTENT, options);
    assert!(result.is_err(), "Expected unsupported user agent to fail");
}

#[test]
fn test_screenshot_options_trace_id() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        trace_id: Some("test-trace-id".to_string()),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_width() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        width: Some(1024),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_height() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        height: Some(768),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_clip() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        clip: Some(true),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_format() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        format: Some(ImageFormat::Jpeg),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_quality() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        quality: Some(85),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_omit_background() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        omit_background: Some(true),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_optimize_for_speed() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        optimize_for_speed: Some(true),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_wait_delay() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        wait_delay: Some(Duration::from_secs(1)),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_wait_for_expression() {
    let html_content: &str = r#"
    <!doctype html>
    <html>
        <head><title>My PDF</title></head>
        <body>
            <h1>Hello, PDF!</h1>
            <script>
                setTimeout(() => {
                    window.isReady = true;
                }, 200);
            </script>
        </body>
    </html>
    "#;

    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        wait_for_expression: Some("window.isReady === true".to_string()),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(html_content, options).unwrap();
}

#[test]
fn test_screenshot_options_emulated_media_type() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        emulated_media_type: Some(MediaType::Screen),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_cookies() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        cookies: Some(vec![Cookie {
            name: "session".to_string(),
            value: "abc123".to_string(),
            domain: "example.com".to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_skip_network_idle_events() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        skip_network_idle_events: Some(false),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_user_agent() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        user_agent: Some("Test-Agent".to_string()),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_extra_http_headers() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        extra_http_headers: Some(
            vec![
                ("Authorization".to_string(), "Bearer token".to_string()),
                ("X-Custom-Header".to_string(), "custom-value".to_string()),
            ]
            .into_iter()
            .collect(),
        ),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_fail_on_http_status_codes() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        fail_on_http_status_codes: Some(vec![404, 500]),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_fail_on_resource_http_status_codes() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        fail_on_resource_http_status_codes: Some(vec![403, 502]),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_fail_on_resource_loading_failed() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        fail_on_resource_loading_failed: Some(true),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_screenshot_options_fail_on_console_exceptions() {
    let client = Client::new(BASE_URL);

    let options = ScreenshotOptions {
        fail_on_console_exceptions: Some(true),
        ..Default::default()
    };

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).unwrap();
}

#[test]
fn test_doc_options_trace_id() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        trace_id: Some("some-trace-id".to_string()),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_password() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        password: Some("secure-password".to_string()),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc(
            "example.odt",
            PASSWORD_PROTECTED_ODT_CONTENT.to_vec(),
            options,
        )
        .unwrap();
}

#[test]
fn test_doc_options_landscape() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        landscape: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_form_fields() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_form_fields: Some(false),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_allow_duplicate_field_names() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        allow_duplicate_field_names: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_bookmarks() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_bookmarks: Some(false),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_notes() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_notes: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_quality() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        quality: Some(75),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_max_image_resolution() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        max_image_resolution: Some(600),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_pdfua() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        pdfua: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_native_page_ranges() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        native_page_ranges: Some("1-3,5".parse().unwrap()),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_bookmarks_to_pdf_destination() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_bookmarks_to_pdf_destination: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_placeholders() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_placeholders: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_notes_pages() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_notes_pages: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_only_notes_pages() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_only_notes_pages: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_notes_in_margin() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_notes_in_margin: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_convert_ooo_target_to_pdf_target() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        convert_ooo_target_to_pdf_target: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_links_relative_fsys() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_links_relative_fsys: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_export_hidden_slides() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        export_hidden_slides: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_skip_empty_pages() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        skip_empty_pages: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_add_original_document_as_stream() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        add_original_document_as_stream: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_single_page_sheets() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        single_page_sheets: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_lossless_image_compression() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        lossless_image_compression: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_reduce_image_resolution() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        reduce_image_resolution: Some(true),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_doc_options_pdfa() {
    let client = Client::new(BASE_URL);

    let options = DocumentOptions {
        pdfa: Some(PDFFormat::A1b),
        ..Default::default()
    };

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .unwrap();
}

#[test]
fn test_pdf_metadata() {
    let client = Client::new(BASE_URL);
    let options = DocumentOptions::default();

    // Create the PDF
    let pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options.clone())
        .unwrap();

    // Update the metadata
    let metadata = HashMap::from([
        ("Title".to_string(), "Test Document 123".into()),
        ("Author".to_string(), "Test Author 123".into()),
    ]);

    let pdf_content = client
        .write_metadata(pdf_content.to_vec(), metadata)
        .unwrap();

    // Read the metadata
    let metadata = client.read_metadata(pdf_content.to_vec()).unwrap();

    assert_eq!(
        metadata.get("Title"),
        Some(&serde_json::Value::String("Test Document 123".to_string()))
    );
    assert_eq!(
        metadata.get("Author"),
        Some(&serde_json::Value::String("Test Author 123".to_string()))
    );
}

#[test]
pub fn test_health_check() {
    let client = Client::new(BASE_URL);
    let _health = client.health_check().unwrap();
}

#[test]
pub fn test_version_string() {
    let client = Client::new(BASE_URL);
    let version = client.version().unwrap();

    // It should start with 8.
    assert!(version.starts_with("8."));
}

#[test]
pub fn test_metrics() {
    let client = Client::new(BASE_URL);
    let _metrics = client.metrics().unwrap();
}
