# wkhtmlapp
## _Bindings to wkhtmltopdf and wkhtmltoimage_

wkhtmlapp makes use of wkhtmltopdf and wkhtmltoimage, command line tools to generate PDF and Images. The library when using command lines to use wkhtmltox allows it to be instantiated the number of times required without causing any errors. This library was developed inspired by barryvdh's laravel-snappy.

Resource  | Link
----- | -----
Crate | [![Crates.io](https://img.shields.io/crates/v/wkhtmltopdf.svg?maxAge=2592000)](https://crates.io/crates/wkhtmltopdf)
Documentation | [Cargo docs](https://github.com/JoelTorresAr/wkhtmlapp.git)
Upstream | [wkhtmltopdf.org](http://wkhtmltopdf.org/)
Wkhtmltox Version | wkhtmltox-0.12.6-1
-----

## Features

- Convert html code to PDF and IMG
- Convert html file to PDF and IMG
- Convert url link to PDF and IMG

## Version Control

 - V.0.1.3 return full file path 


## Example

```sh
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
}
```

## PDF Examples

```sh
let pdf_app = PdfApp::new().expect("Failed to init PDF Application");
let html_code = r#"<html><body><div>DEMO</div></body></html>"#.to_string();
let file_path = pdf_app.run(WkhtmlInput::Html(html_code),"demo").unwrap();
let file_path = pdf_app.run(WkhtmlInput::File("examples/index.html".to_string()), "demo").unwrap();
let file_path = pdf_app.run(
            WkhtmlInput::Url("https://www.rust-lang.org/en-US/".to_string()),
            "demo",
        ).unwrap();
```
## IMG Examples

```sh
let mut image_app = ImgApp::new().expect("Failed to init image Application");

        let file_path = image_app
            .set_format(ImgFormat::Png)
            .unwrap()
            .run(WkhtmlInput::File("examples/index.html".to_string()), "demo").unwrap();

        let file_path = image_app.run(
            WkhtmlInput::Url("https://www.rust-lang.org/en-US/".to_string()),
            "demo",
        ).unwrap();
```

## Args

```sh
let mut image_app = ImgApp::new().expect("Failed to init image Application");

        let mut pdf_app = PdfApp::new().expect("Failed to init PDF Application");
        pdf_app.set_arg("margin-top", Some("0".to_string())).unwrap();

        // Test building PDF from HTML
        let html_code = r#"<html><body><div>DEMO</div></body></html>"#.to_string();
        let file_path = pdf_app.run(WkhtmlInput::Html(html_code), "demo").unwrap();
```

## License

MIT

**Free Software, Hell Yeah!**