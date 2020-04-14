#![allow(missing_docs)]

pub mod playpen_editor;

#[cfg(feature = "search")]
pub mod searcher;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::errors::*;

pub static INDEX: &[u8] = include_bytes!("index.hbs");
pub static HEADER: &[u8] = include_bytes!("header.hbs");
pub static CHROME_CSS: &[u8] = include_bytes!("css/chrome.css");
pub static GENERAL_CSS: &[u8] = include_bytes!("css/general.css");
pub static FONTS_CSS: &[u8] = include_bytes!("css/fonts.css");
pub static PRINT_CSS: &[u8] = include_bytes!("css/print.css");
pub static VARIABLES_CSS: &[u8] = include_bytes!("css/variables.css");
pub static FAVICON: &[u8] = include_bytes!("favicon.png");
pub static JS: &[u8] = include_bytes!("book.js");
pub static HIGHLIGHT_JS: &[u8] = include_bytes!("highlight.js");
pub static TOMORROW_NIGHT_CSS: &[u8] = include_bytes!("tomorrow-night.css");
pub static HIGHLIGHT_CSS: &[u8] = include_bytes!("highlight.css");
pub static AYU_HIGHLIGHT_CSS: &[u8] = include_bytes!("ayu-highlight.css");
pub static CLIPBOARD_JS: &[u8] = include_bytes!("clipboard.min.js");
pub static FONT_AWESOME: &[u8] = include_bytes!("FontAwesome/css/font-awesome.min.css");
pub static FONT_AWESOME_EOT: &[u8] = include_bytes!("FontAwesome/fonts/fontawesome-webfont.eot");
pub static FONT_AWESOME_SVG: &[u8] = include_bytes!("FontAwesome/fonts/fontawesome-webfont.svg");
pub static FONT_AWESOME_TTF: &[u8] = include_bytes!("FontAwesome/fonts/fontawesome-webfont.ttf");
pub static FONT_AWESOME_WOFF: &[u8] = include_bytes!("FontAwesome/fonts/fontawesome-webfont.woff");
pub static FONT_AWESOME_WOFF2: &[u8] =
    include_bytes!("FontAwesome/fonts/fontawesome-webfont.woff2");
pub static FONT_AWESOME_OTF: &[u8] = include_bytes!("FontAwesome/fonts/FontAwesome.otf");

// An array of (file_name, file_contents) pairs
pub static FONT_OPEN_SANS: [(&str, &[u8]); 20] = [
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300.woff2")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300italic.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300italic.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300italic.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-300italic.woff2")
    ),

    // Regular is 400
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-regular.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-regular.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-regular.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-regular.woff2")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-italic.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-italic.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-italic.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-italic.woff2")
    ),

    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600.woff2")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600italic.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600italic.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600italic.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-600italic.woff2")
    ),

    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700.woff2")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700italic.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700italic.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700italic.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-700italic.woff2")
    ),

    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800.woff2")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800italic.woff", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800italic.woff")
    ),
    (
        "fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800italic.woff2", 
        include_bytes!("fonts/open-sans-v17-latin_vietnamese_latin-ext_greek-ext_greek_cyrillic-ext_cyrillic-800italic.woff2")
    ),
];

// An array of (file_name, file_contents) pairs
pub static FONT_SOURCE_CODE_PRO: [(&str, &[u8]); 2] = [
    (
        "fonts/source-code-pro-v11-latin_vietnamese_latin-ext_greek_cyrillic-ext_cyrillic-500.woff",
        include_bytes!("fonts/source-code-pro-v11-latin_vietnamese_latin-ext_greek_cyrillic-ext_cyrillic-500.woff"),
    ),
    (
        "fonts/source-code-pro-v11-latin_vietnamese_latin-ext_greek_cyrillic-ext_cyrillic-500.woff2",
        include_bytes!("fonts/source-code-pro-v11-latin_vietnamese_latin-ext_greek_cyrillic-ext_cyrillic-500.woff2"),
    ),
];

/// The `Theme` struct should be used instead of the static variables because
/// the `new()` method will look if the user has a theme directory in their
/// source folder and use the users theme instead of the default.
///
/// You should only ever use the static variables directly if you want to
/// override the user's theme with the defaults.
#[derive(Debug, PartialEq)]
pub struct Theme {
    pub index: Vec<u8>,
    pub header: Vec<u8>,
    pub chrome_css: Vec<u8>,
    pub fonts_css: Vec<u8>,
    pub general_css: Vec<u8>,
    pub print_css: Vec<u8>,
    pub variables_css: Vec<u8>,
    pub favicon: Vec<u8>,
    pub js: Vec<u8>,
    pub highlight_css: Vec<u8>,
    pub tomorrow_night_css: Vec<u8>,
    pub ayu_highlight_css: Vec<u8>,
    pub highlight_js: Vec<u8>,
    pub clipboard_js: Vec<u8>,
}

impl Theme {
    /// Creates a `Theme` from the given `theme_dir`.
    /// If a file is found in the theme dir, it will override the default version.
    pub fn new<P: AsRef<Path>>(theme_dir: P) -> Self {
        let theme_dir = theme_dir.as_ref();
        let mut theme = Theme::default();

        // If the theme directory doesn't exist there's no point continuing...
        if !theme_dir.exists() || !theme_dir.is_dir() {
            return theme;
        }

        // Check for individual files, if they exist copy them across
        {
            let files = vec![
                (theme_dir.join("index.hbs"), &mut theme.index),
                (theme_dir.join("header.hbs"), &mut theme.header),
                (theme_dir.join("book.js"), &mut theme.js),
                (theme_dir.join("css/chrome.css"), &mut theme.chrome_css),
                (theme_dir.join("css/fonts.css"), &mut theme.fonts_css),
                (theme_dir.join("css/general.css"), &mut theme.general_css),
                (theme_dir.join("css/print.css"), &mut theme.print_css),
                (
                    theme_dir.join("css/variables.css"),
                    &mut theme.variables_css,
                ),
                (theme_dir.join("favicon.png"), &mut theme.favicon),
                (theme_dir.join("highlight.js"), &mut theme.highlight_js),
                (theme_dir.join("clipboard.min.js"), &mut theme.clipboard_js),
                (theme_dir.join("highlight.css"), &mut theme.highlight_css),
                (
                    theme_dir.join("tomorrow-night.css"),
                    &mut theme.tomorrow_night_css,
                ),
                (
                    theme_dir.join("ayu-highlight.css"),
                    &mut theme.ayu_highlight_css,
                ),
            ];

            for (filename, dest) in files {
                if !filename.exists() {
                    continue;
                }

                if let Err(e) = load_file_contents(&filename, dest) {
                    warn!("Couldn't load custom file, {}: {}", filename.display(), e);
                }
            }
        }

        theme
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            index: INDEX.to_owned(),
            header: HEADER.to_owned(),
            chrome_css: CHROME_CSS.to_owned(),
            fonts_css: FONTS_CSS.to_owned(),
            general_css: GENERAL_CSS.to_owned(),
            print_css: PRINT_CSS.to_owned(),
            variables_css: VARIABLES_CSS.to_owned(),
            favicon: FAVICON.to_owned(),
            js: JS.to_owned(),
            highlight_css: HIGHLIGHT_CSS.to_owned(),
            tomorrow_night_css: TOMORROW_NIGHT_CSS.to_owned(),
            ayu_highlight_css: AYU_HIGHLIGHT_CSS.to_owned(),
            highlight_js: HIGHLIGHT_JS.to_owned(),
            clipboard_js: CLIPBOARD_JS.to_owned(),
        }
    }
}

/// Checks if a file exists, if so, the destination buffer will be filled with
/// its contents.
fn load_file_contents<P: AsRef<Path>>(filename: P, dest: &mut Vec<u8>) -> Result<()> {
    let filename = filename.as_ref();

    let mut buffer = Vec::new();
    File::open(filename)?.read_to_end(&mut buffer)?;

    // We needed the buffer so we'd only overwrite the existing content if we
    // could successfully load the file into memory.
    dest.clear();
    dest.append(&mut buffer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::Builder as TempFileBuilder;

    #[test]
    fn theme_uses_defaults_with_nonexistent_src_dir() {
        let non_existent = PathBuf::from("/non/existent/directory/");
        assert!(!non_existent.exists());

        let should_be = Theme::default();
        let got = Theme::new(&non_existent);

        assert_eq!(got, should_be);
    }

    #[test]
    fn theme_dir_overrides_defaults() {
        let files = [
            "index.hbs",
            "header.hbs",
            "favicon.png",
            "css/chrome.css",
            "css/fonts.css",
            "css/general.css",
            "css/print.css",
            "css/variables.css",
            "book.js",
            "highlight.js",
            "tomorrow-night.css",
            "highlight.css",
            "ayu-highlight.css",
            "clipboard.min.js",
        ];

        let temp = TempFileBuilder::new().prefix("mdbook-").tempdir().unwrap();
        fs::create_dir(temp.path().join("css")).unwrap();

        // "touch" all of the special files so we have empty copies
        for file in &files {
            File::create(&temp.path().join(file)).unwrap();
        }

        let got = Theme::new(temp.path());

        let empty = Theme {
            index: Vec::new(),
            header: Vec::new(),
            chrome_css: Vec::new(),
            fonts_css: Vec::new(),
            general_css: Vec::new(),
            print_css: Vec::new(),
            variables_css: Vec::new(),
            favicon: Vec::new(),
            js: Vec::new(),
            highlight_css: Vec::new(),
            tomorrow_night_css: Vec::new(),
            ayu_highlight_css: Vec::new(),
            highlight_js: Vec::new(),
            clipboard_js: Vec::new(),
        };

        assert_eq!(got, empty);
    }
}
