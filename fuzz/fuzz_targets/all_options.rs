#![no_main]

use libfuzzer_sys::fuzz_target;

use comrak::{
    markdown_to_html, ExtensionOptions, Options, ParseOptions,
    RenderOptions, ListStyleType,
};

fuzz_target!(|s: &str| {
    let mut extension = ExtensionOptions::default();
    extension.strikethrough = true;
    extension.tagfilter = true;
    extension.table = true;
    extension.autolink = true;
    extension.tasklist = true;
    extension.superscript = true;
    extension.header_ids = Some("user-content-".to_string());
    extension.footnotes = true;
    extension.description_lists = true;
    extension.front_matter_delimiter = Some("---".to_string());
    extension.shortcodes = true;

    let mut parse = ParseOptions::default();
    parse.smart = true;
    parse.default_info_string = Some("rust".to_string());
    parse.relaxed_tasklist_matching = true;
    parse.relaxed_autolinks = true;

    let mut render = RenderOptions::default();
    render.hardbreaks = true;
    render.github_pre_lang = true;
    render.full_info_string = true;
    render.width = 80;
    render.unsafe_ = true;
    render.escape = true;
    render.list_style = ListStyleType::Star;
    render.sourcepos = true;

    markdown_to_html(
        s,
        &Options { extension, parse, render },
    );
});
