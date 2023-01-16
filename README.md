# wkhtmlapp
## _Bindings to wkhtmltopdf and wkhtmltoimage_


wkhtmlapp depends on the wkhtmltopdf application to generate PDFs or images, abstracted in such a 
way that it can support multithreading without damaging the main instance. 
This library was developed inspired by barryvdh's laravel-snappy.

| Resource          | Link                                                                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Crate             | [![Crates.io](https://img.shields.io/crates/v/wkhtmlapp?color=warning&style=plastic)](https://crates.io/crates/wkhtmlapp) |
| Documentation     | [Cargo docs](https://github.com/JoelTorresAr/wkhtmlapp.git)                                                               |
| Upstream          | [wkhtmltopdf.org](http://wkhtmltopdf.org/)                                                                                |
| Wkhtmltox Version | [wkhtmltox-0.12.6-1](https://github.com/wkhtmltopdf/packaging/releases)                                                   |
-----
## _Required setup before use_
 - set wkhtmltopdf in path system environment variables or download the portable versions 
   of wkhtmltopdf for your operating system, store them and reference it in the .env file 
   using WKHTMLTOPDF_CMD and WKHTMLTOIMG_CMD respectively.
```sh
   //.env
   WKHTMLAPP_WORK_DIR="storage/temp"
   WKHTMLTOPDF_CMD="assets/bin/wkhtmltopdf/0.16/wkhtmltopdf"
   WKHTMLTOIMG_CMD="assets/bin/wkhtmltopdf/0.16/wkhtmltoimage"
```

### ONLY SOME ARGUMENTS HAVE BEEN VERIFIED, USE WITH CAUTION

## Features

- Convert html code to PDF and IMG
- Convert html file to PDF and IMG
- Convert url link to PDF and IMG

##  _Change Logs_

### 1.0.0
 - Successfully tested with actix-web on windows and linux systems

### 0.2.0
 - You can instantiate PdfApp or ImgApp which will only serve to generate pdf's or images 
   respectively or instantiate App which gives you access to both

### 0.1.5
 - Arg value is no longer optional

### 0.1.4

 - Lifetime was implemented, allowing to use str instead of String in sending arguments
### 0.1.3

 - Fix: Return full file path


## Example

```sh
use wkhtmlapp::WkhtmlError;
fn main() -> Result<(), WkhtmlError> {
    let mut app = wkhtmlapp::App::new()?;
    let app_report = app.pdf_app
        .set_arg("enable-smart-shrinking", "true")?
        .set_arg("title", "Torres")?
        .set_arg("header-right", "Página [page] de [toPage]")?
        .set_arg("margin-top", "18")?;

    let report = app_report.run(
        wkhtmlapp::WkhtmlInput::Url("https://www.w3schools.com/graphics/svg_intro.asp"),
        "demo",
    )?;
    println!("report: {:?}", report);
    Ok(())
}
```

## PDF Examples

```sh
let pdf_app = PdfApp::new().expect("Failed to init PDF Application");
let html_code = r#"<html><body><div>DEMO</div></body></html>"#;
let file_path = pdf_app.run(WkhtmlInput::Html(html_code),"demo")?;
let file_path = pdf_app.run(WkhtmlInput::File("examples/index.html"), "demo")?;
let file_path = pdf_app.run(
            WkhtmlInput::Url("https://www.rust-lang.org/en-US/"),
            "demo",
        )?;
```
## IMG Examples

```sh
        let mut image_app = ImgApp::new().expect("Failed to init image Application");
        let args = HashMap::from([("height", "20"), ("width", "20")]);

        let res = image_app
            .set_format(ImgFormat::Png)?
            .set_args(args)?
            .run(WkhtmlInput::File("examples/index.html"), "demo")?;
```
##  _ImgApp Args_
| Option                       | Description                                                                                                  |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------ |
| allow                        | Allow the file or files from the specified folder to be loaded (repeatable)                                  |
| bypass-proxy-for             | Bypass proxy for host (repeatable)                                                                           |
| cache-dir                    | Web cache directory                                                                                          |
| checkbox-checked-svg         | Use this SVG file when rendering checked checkboxes                                                          |
| checked-svg                  | Use this SVG file when rendering unchecked checkboxes                                                        |
| cookie                       | Set an additional cookie (repeatable)                                                                        |
| cookie-jar                   | Read and write cookies from and to the supplied cookie jar file                                              |
| crop-h                       | Set height for cropping                                                                                      |
| crop-w                       | Set width for cropping                                                                                       |
| crop-x                       | Set x coordinate for cropping (default 0)                                                                    |
| crop-y                       | Set y coordinate for cropping (default 0)                                                                    |
| custom-header                | Set an additional HTTP header (repeatable)                                                                   |
| custom-header-propagation    | Add HTTP headers specified by --custom-header for each resource request.                                     |
| no-custom-header-propagation | Do not add HTTP headers specified by --custom-header for each resource request.                              |
| debug-javascript             | Show javascript debugging output                                                                             |
| no-debug-javascript          | Do not show javascript debugging output (default)                                                            |
| encoding                     | Set the default text encoding, for input                                                                     |
| format                       | Output format                                                                                                |
| height                       | Set screen height (default is calculated from page content) (default 0)                                      |
| images                       | Do load or print images (default)                                                                            |
| no-images                    | Do not load or print images                                                                                  |
| disable-javascript           | Do not allow web pages to run javascript                                                                     |
| enable-javascript            | Do allow web pages to run javascript (default)                                                               |
| javascript-delay             | Wait some milliseconds for javascript finish (default 200)                                                   |
| load-error-handling          | Specify how to handle pages that fail to load: abort, ignore or skip (default abort)                         |
| load-media-error-handling    | Specify how to handle media files that fail to load: abort, ignore or skip (default ignore)                  |
| disable-local-file-access    | Do not allowed conversion of a local file to read in other local files, unless explicitly allowed with allow |
| enable-local-file-access     | Allowed conversion of a local file to read in other local files. (default)                                   |
| minimum-font-size            | Minimum font size                                                                                            |
| password                     | HTTP Authentication password                                                                                 |
| disable-plugins              | Disable installed plugins (default)                                                                          |
| enable-plugins               | Enable installed plugins (plugins will likely not work)                                                      |
| post                         | Add an additional post field                                                                                 |
| post-file                    | Post an additional file                                                                                      |
| proxy                        | Use a proxy                                                                                                  |
| quality                      | Output image quality (between 0 and 100) (default 94)                                                        |
| quiet                        | Be less verbose                                                                                              |
| radiobutton-checked-svg      | Use this SVG file when rendering checked radio-buttons                                                       |
| radiobutton-svg              | Use this SVG file when rendering unchecked radio-buttons                                                     |
| run-script                   | Run this additional javascript after the page is done loading (repeatable)                                   |
| disable-smart-width          | Use the specified width even if it is not large enough for the content                                       |
| enable-smart-width           | Extend --width to fit unbreakable content (default)                                                          |
| stop-slow-scripts            | Stop slow running javascript                                                                                 |
| no-stop-slow-scripts         | Do not stop slow running javascript (default)                                                                |
| transparent                  | Make the background transparent in pngs *                                                                    |
| use-xserver                  | Use the X server (some plugins and other stuff might not work without X11)                                   |
| user-style-sheet             | Specify a user style sheet, to load with every page                                                          |
| username                     | HTTP Authentication username                                                                                 |
| width                        | Set screen width (default is 1024)                                                                           |
| window-status                | Wait until window.status is equal to this string before rendering page                                       |
| zoom                         | Use this zoom factor (default 1)                                                                             |


##  _PdfApp Args_
| Option                       | Description                |
| ---------------------------- | -------------------------- |
| collate                      | Global options             |
| no-collate                   | Global options             |
| cookie-jar                   | Global options             |
| copies                       | Global options             |
| dpi                          | Global options             |
| extended-help                | Global options             |
| grayscale                    | Global options             |
| help                         | Global options             |
| htmldoc                      | Global options             |
| ignore-load-errors           | Global options - old v0.9  |
| image-dpi                    | Global options             |
| image-quality                | Global options             |
| license                      | Global options             |
| log-level                    | Global options             |
| lowquality                   | Global options             |
| manpage                      | Global options             |
| margin-bottom                | Global options             |
| margin-left                  | Global options             |
| margin-right                 | Global options             |
| margin-top                   | Global options             |
| orientation                  | Global options             |
| page-height                  | Global options             |
| page-size                    | Global options             |
| page-width                   | Global options             |
| no-pdf-compression           | Global options             |
| quiet                        | Global options             |
| read-args-from-stdin         | Global options             |
| readme                       | Global options             |
| title                        | Global options             |
| use-xserver                  | Global options             |
| version                      | Global options             |
| dump-default-toc-xsl         | Outline options            |
| dump-outline                 | Outline options            |
| outline                      | Outline options            |
| no-outline                   | Outline options            |
| outline-depth                | Outline options            |
| output-format                | Outline options            |
| allow                        | Page options               |
| background                   | Page options               |
| no-background                | Page options               |
| bypass-proxy-for             | Page options               |
| cache-dir                    | Page options               |
| checkbox-checked-svg         | Page options               |
| checkbox-svg                 | Page options               |
| cookie                       | Page options               |
| custom-header                | Page options               |
| custom-header-propagation    | Page options               |
| no-custom-header-propagation | Page options               |
| debug-javascript             | Page options               |
| no-debug-javascript          | Page options               |
| default-header               | Page options               |
| encoding                     | Page options               |
| disable-external-links       | Page options               |
| enable-external-links        | Page options               |
| disable-forms                | Page options               |
| enable-forms                 | Page options               |
| images                       | Page options               |
| no-images                    | Page options               |
| disable-internal-links       | Page options               |
| enable-internal-links        | Page options               |
| disable-javascript           | Page options               |
| enable-javascript            | Page options               |
| javascript-delay             | Page options               |
| keep-relative-links          | Page options               |
| load-error-handling          | Page options               |
| load-media-error-handling    | Page options               |
| disable-local-file-access    | Page options               |
| enable-local-file-access     | Page options               |
| minimum-font-size            | Page options               |
| exclude-from-outline         | Page options               |
| include-in-outline           | Page options               |
| page-offset                  | Page options               |
| password                     | Page options               |
| disable-plugins              | Page options               |
| enable-plugins               | Page options               |
| post                         | Page options               |
| post-file                    | Page options               |
| print-media-type             | Page options               |
| no-print-media-type          | Page options               |
| proxy                        | Page options               |
| proxy-hostname-lookup        | Page options               |
| radiobutton-checked-svg      | Page options               |
| radiobutton-svg              | Page options               |
| redirect-delay               | Page options // old v0.9   |
| resolve-relative-links       | Page options               |
| run-script                   | Page options               |
| disable-smart-shrinking      | Page options               |
| enable-smart-shrinking       | Page options               |
| ssl-crt-path                 | Page options               |
| ssl-key-password             | Page options               |
| ssl-key-path                 | Page options               |
| stop-slow-scripts            | Page options               |
| no-stop-slow-scripts         | Page options               |
| disable-toc-back-links       | Page options               |
| enable-toc-back-links        | Page options               |
| user-style-sheet             | Page options               |
| username                     | Page options               |
| viewport-size                | Page options               |
| window-status                | Page options               |
| zoom                         | Page options               |
| footer-center                | Headers and footer options |
| footer-font-name             | Headers and footer options |
| footer-font-size             | Headers and footer options |
| footer-html                  | Headers and footer options |
| footer-left                  | Headers and footer options |
| footer-line                  | Headers and footer options |
| no-footer-line               | Headers and footer options |
| footer-right                 | Headers and footer options |
| footer-spacing               | Headers and footer options |
| header-center                | Headers and footer options |
| header-font-name             | Headers and footer options |
| header-font-size             | Headers and footer options |
| header-html                  | Headers and footer options |
| header-left                  | Headers and footer options |
| header-line                  | Headers and footer options |
| no-header-line               | Headers and footer options |
| header-right                 | Headers and footer options |
| header-spacing               | Headers and footer options |
| replace                      | Headers and footer options |
| cover                        | Cover object               |
| toc                          | TOC object                 |
| disable-dotted-lines         | TOC options                |
| toc-depth                    | TOC options -  old v0.9    |
| toc-font-name                | TOC options -  old v0.9    |
| toc-l1-font-size             | TOC options -  old v0.9    |
| toc-header-text              | TOC options                |
| toc-header-font-name         | TOC options -  old v0.9    |
| toc-header-font-size         | TOC options -  old v0.9    |
| toc-level-indentation        | TOC options                |
| disable-toc-links            | TOC options                |
| toc-text-size-shrink         | TOC options                |
| xsl-style-sheet              | TOC options                |

## License

MIT

**Free Software, Hell Yeah!**