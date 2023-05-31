// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn gfm_strikethrough_test_1() {
    let original = r##"~~Hi~~ Hello, ~there~ world!
"##;
    let expected = r##"<p><del>Hi</del> Hello, <del>there</del> world!</p>
"##;

    test_markdown_html(original, expected, false, false, true);
}

#[test]
fn gfm_strikethrough_test_2() {
    let original = r##"This ~~has a

new paragraph~~.
"##;
    let expected = r##"<p>This ~~has a</p>
<p>new paragraph~~.</p>
"##;

    test_markdown_html(original, expected, false, false, true);
}

#[test]
fn gfm_strikethrough_test_3() {
    let original = r##"This will ~~~not~~~ strike.
"##;
    let expected = r##"<p>This will ~~~not~~~ strike.</p>
"##;

    test_markdown_html(original, expected, false, false, true);
}
