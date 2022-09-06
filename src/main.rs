fn main() {
    let mut app = wkhtmlapp::PdfApp::new().unwrap();
    let app_report = app.set_arg("enable-smart-shrinking", Some("true".to_string())).unwrap();
    let html = r#"<html><body><div>foo</div></body></html>"#;
    let report = app_report
        .run(
            wkhtmlapp::app::WkhtmlInput::Url(
                "https://www.w3schools.com/graphics/svg_intro.asp".to_string(),
            ),
            "demo",
        )
        .unwrap();
        println!("{:?}", report);
}