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

use crate::fzf::prompt_user_selection;
use open;
use skim::prelude::*;
use titlecase::titlecase;

struct TwDocItem {
    url_path: String,
}

impl SkimItem for TwDocItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.url_path)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        let text = titlecase(&self.url_path.replace("-", " "));

        let result = format!("\u{001b}[38;5;253m{}", text);

        AnsiString::parse(&result)
    }
}

pub fn search_and_open() {
    let items = TW_DOCS_OPTIONS
        .split("\n")
        .map(|url_path| TwDocItem {
            url_path: url_path.to_string(),
        })
        .map(|item| Box::new(item) as Box<dyn SkimItem>);

    if let Some(selection) = prompt_user_selection(Box::new(items)) {
        let tw_url = format!("https://tailwindcss.com/docs/{}", selection);

        println!("Opening...");
        if let Err(e) = open::that(tw_url) {
            panic!("Failed to open tailwind documentation site. Reason: {e}");
        }
    }
}
