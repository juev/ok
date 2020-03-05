pub fn print_usage(verbose: bool) {
    println!("Usage: ok [options] <number> [script-arguments..]
       ok command [options]
       
command (use one):
  <number>            Run the <number>th command from the '.ok' file.
  l, list             Show the list from the '.ok' file. Default command.
  L, list-once        Same as list, but only show when pwd is different from when the list was last shown.
  p, list-prompt      Show the list and wait for input at the ok-prompt (like --list and <number> in one command).
  h, help             Show this usage page.

options:
  -c, --comment_align N  Level of comment alignment. See $_OK_COMMENT_ALIGN
  -v, --verbose       Show more output, mostly errors. Also it shows environment-variables in this screen.
  -q, --quiet         Only show really necessary output, so surpress echoing the command.
  -f, --file <file>   Use a custom file instead of '.ok'; use '-' for stdin
  -a, --alias <name>  When using 'ok' in an alias, <name> is used to keep the history correct when used with 'list-prompt'.
  -V, --version       Show version number and exit
  -h, --help          Show this help screen
script-arguments:
  ...                 These are passed through, when a line is executed (you can enter these too at the ok-prompt)");

    if verbose {
        println!("environment variables (used for colored output; current colors are shown):
  _OK_C_HEADING      Color-code for lines starting with a comment (heading). Defaults to red.
  _OK_C_NUMBER       Color-code for numbering. Defaults to cyan.
  _OK_C_COMMENT      Color-code for comments after commands. Defaults to blue.
  _OK_C_COMMAND      Color-code for commands. Defaults to color-reset.
  _OK_C_PROMPT       Color-code for prompt (both input as command confirmation). Defaults to color for numbering.
environment variables (other configuration):
  _OK_COMMENT_ALIGN  Level (unset) of comment alignment. 0=no alignment, 1=align consecutive lines (Default), 2=including whitespace, 3 align all.
  _OK_PROMPT         String (unset) used as prompt (both input as command confirmation). Defaults to '$ '.
  _OK_PROMPT_DEFAULT Setting (unset) if the prompt is default shown. 1=use command list-prompt when issuing no command, otherwise use list.
  _OK_VERBOSE        Level (unset) of feedback ok provides. 0=quiet, 1=normal, 2=verbose. Defaults to 1. Can be overriden with --verbose or --quiet.
environment variables (for internal use):
  _OK__LAST_PWD      Remember the path (/path/to/some/place/with/an/.ok/file) that was last listed, for use with the list-once command.
  _OK__PATH_TO_ME    The path (/path/to/ok-bash) to the location of this script.");
    }
}
