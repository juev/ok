# ok

[![Crates.io](https://img.shields.io/crates/v/ok)](https://crates.io/crates/ok) [![Crates.io](https://img.shields.io/crates/l/ok.svg?maxAge=2592000)](https://github.com/juev/ok/blob/master/LICENSE)

This is Rust fork of [ok-bash](https://github.com/secretGeek/ok-bash).

## "ok" gives you .ok folder profiles for bash

`ok` makes you smarter and more efficient.

Do you work on many different projects? And in each project, are there commands you use that are specific to that project? You need a `.ok` file.

An `.ok` file holds a bunch of handy one-liners, specific to the folder it is in. It can be viewed with a simple command. Any command can be executed with the command `ok <number>` (example, `ok 3` to run the 3rd command.)

Imagine your `.ok` file contains these three lines:

    ./build.sh # builds the project
    ./deploy.sh # deploys the project
    ./commit_push.sh "$1" # commit with comment, rebase and push

You can run those commands with "`ok 1`", "`ok 2`" or "`ok 3 'oops!'`", respectively.

An `.ok` file acts as a neat place to document how a given project works. This is useful if you have many projects, or many people working on a project. It's such a little file; it's so quick to write and so easy to edit.

It's better than normal documentation: it's executable.

If you run the command `ok` (with no parameters) you'll see the file listed, with numbers against each command:

    $ ok
    1. ./build.sh            # builds the project
    2. ./deploy.sh           # deploys the project
    3. ./commit_push.sh "$1" # commit with comment, rebase and push

(It will also be stylishly formatted, to make it easier to read at a glance)

Then if you run `ok <number>` (ok followed by a number) you'll execute that line of the file.

    $ ok 1
    $ ./build.sh # builds the project
    building.....

And you can pass simple arguments to the commands. For example:

    $ ok 3 "Added laser guidance system"
    $ ./commit_push.sh "$1" # commit with comment, rebase and push

    Committing with comment "Added laser guidance system"
    Commit succeeded.
    Rebase successful
    Pushing to master.


## Getting started

### Installation

Put binary file from [release](https://github.com/juev/ok/releases/latest) page to your `PATH`.

### First steps after installing

You can try out the included `.ok` file by navigating to `~/path/to/ok` and type `ok`. Explore some of the options.

Next you can create your own `.ok` file. Navigate to any folder where you want to use `ok`, and run for example:

    echo '# My first ok-command'>>.ok
    echo 'echo "Hi $USER, the time when pressed enter was $(date "+%H:%M:%S")"'>>.ok

The first line adds a "heading" to the `.ok` file, which is nice to keep the file organized. I used append redirect (`>>.ok`) to append a line to the `.ok` file. When the file doesn't exist, it's created.

Also, I use single quotes `'`, so no funny things happen to the string, before it ends up in your `.ok` file. This way, `$USER` and `$(date...)` are evaluated when the `ok` command is run, not when you add the line to the `.ok` file.

What to put in these `.ok` files? A good place to start is the projects documentation: search for all commands that are buried in there. Even add running a script file with a comment (and grouped under the correct heading) can be really helpfull. And whenever you `man` a command or search Google for it, remember to check if it's worth to add it to your `.ok` file. It probably is. And it's easy to remove again.

After that you can look at customization. This allows you to do things such as:

* show the ok-list automatically everytime you change folders
* change the coloring scheme and other formatting options
* create your own commands that use ok
