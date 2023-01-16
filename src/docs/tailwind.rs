// This is done via the following JS code in the browser
//
// const snapshot = document.evaluate(
//  "//a[contains(@href,'docs') and contains(@class, 'block')]",
//  document,
//  null,
//  XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,
//  null
// );
//
// const items = [...Array(snapshot.snapshotLength)].map((_, i) => snapshot.snapshotItem(i));
// const options = items.map((item) => item.href).map((path) => path.split("/").at(-1)).join("\n");
pub const TW_DOCS_OPTIONS: &str = "installation\neditor-setup\nusing-with-preprocessors\noptimizing-for-production\nbrowser-support\nupgrade-guide\nutility-first\nhover-focus-and-other-states\nresponsive-design\ndark-mode\nreusing-styles\nadding-custom-styles\nfunctions-and-directives\nconfiguration\ncontent-configuration\ntheme\nscreens\ncustomizing-colors\ncustomizing-spacing\nplugins\npresets\npreflight\naspect-ratio\ncontainer\ncolumns\nbreak-after\nbreak-before\nbreak-inside\nbox-decoration-break\nbox-sizing\ndisplay\nfloat\nclear\nisolation\nobject-fit\nobject-position\noverflow\noverscroll-behavior\nposition\ntop-right-bottom-left\nvisibility\nz-index\nflex-basis\nflex-direction\nflex-wrap\nflex\nflex-grow\nflex-shrink\norder\ngrid-template-columns\ngrid-column\ngrid-template-rows\ngrid-row\ngrid-auto-flow\ngrid-auto-columns\ngrid-auto-rows\ngap\njustify-content\njustify-items\njustify-self\nalign-content\nalign-items\nalign-self\nplace-content\nplace-items\nplace-self\npadding\nmargin\nspace\nwidth\nmin-width\nmax-width\nheight\nmin-height\nmax-height\nfont-family\nfont-size\nfont-smoothing\nfont-style\nfont-weight\nfont-variant-numeric\nletter-spacing\nline-height\nlist-style-type\nlist-style-position\ntext-align\ntext-color\ntext-decoration\ntext-decoration-color\ntext-decoration-style\ntext-decoration-thickness\ntext-underline-offset\ntext-transform\ntext-overflow\ntext-indent\nvertical-align\nwhitespace\nword-break\ncontent\nbackground-attachment\nbackground-clip\nbackground-color\nbackground-origin\nbackground-position\nbackground-repeat\nbackground-size\nbackground-image\ngradient-color-stops\nborder-radius\nborder-width\nborder-color\nborder-style\ndivide-width\ndivide-color\ndivide-style\noutline-width\noutline-color\noutline-style\noutline-offset\nring-width\nring-color\nring-offset-width\nring-offset-color\nbox-shadow\nbox-shadow-color\nopacity\nmix-blend-mode\nbackground-blend-mode\nblur\nbrightness\ncontrast\ndrop-shadow\ngrayscale\nhue-rotate\ninvert\nsaturate\nsepia\nbackdrop-blur\nbackdrop-brightness\nbackdrop-contrast\nbackdrop-grayscale\nbackdrop-hue-rotate\nbackdrop-invert\nbackdrop-opacity\nbackdrop-saturate\nbackdrop-sepia\nborder-collapse\nborder-spacing\ntable-layout\ntransition-property\ntransition-duration\ntransition-timing-function\ntransition-delay\nanimation\nscale\nrotate\ntranslate\nskew\ntransform-origin\naccent-color\nappearance\ncursor\ncaret-color\npointer-events\nresize\nscroll-behavior\nscroll-margin\nscroll-padding\nscroll-snap-align\nscroll-snap-stop\nscroll-snap-type\ntouch-action\nuser-select\nwill-change\nfill\nstroke\nstroke-width\nscreen-readers\ntypography-plugin";

use open;
use skim::prelude::*;
use std::io::Cursor;

pub fn search_and_open() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(TW_DOCS_OPTIONS));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let selected_item = selected_items.first().expect("No item was selected");

    println!("Opening...");

    let tw_url = format!("https://tailwindcss.com/docs/{}", selected_item.output());
    if let Err(e) = open::that(tw_url) {
        panic!("Failed to open tailwind documentation site. Reason: {e}");
    }
}
