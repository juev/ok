extern crate getopts;

use getopts::*;

pub fn create_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help screen");
    opts.optflag("c", "comment_align", "N  Level of comment alignment.");
    opts.optflag(
        "v",
        "verbose",
        "Show more output, mostly errors. Also it shows environment-variables in this screen.",
    );
    opts.optflag(
        "q",
        "quiet",
        "Only show really necessary output, so surpress echoing the command.",
    );
    opts.optopt(
        "f",
        "file",
        "Use a custom file instead of '.ok'; use '-' for stdin",
        "file",
    );
    opts.optopt(
        "a", 
        "alias", 
        "When using 'ok' in an alias, <name> is used to keep the history correct when used with 'list-prompt'.", 
        "name");
    opts.optflag("V", "version", "Show version number and exit");
    opts
}
