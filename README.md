# Feeds to Readeck

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://app.codspeed.io/SardonicPelican/feeds-to-readeck?utm_source=badge)

<b>Feeds to Readeck</b> watches your RSS and Atom feeds
and pushes new items to your [Readeck][readeck] instance as bookmarks.

[readeck]: https://readeck.org/

## License

<b>Feeds to Readeck</b> is licensed
under the terms of either the [MIT license][license-mit]
or the [Apache License, version 2.0][license-apache], at your option.
<b>Feeds to Readeck</b> also uses third party libraries,
some of which have different licenses.

### Contribution

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.

[license-mit]: LICENSE-MIT
[license-apache]: LICENSE-APACHE

## Prerequisites

You'll need Cargo, Rust's package manager.
If you don't already have it,
go to the [Rust][rust] home page,
then download and install Rust for your platform,
which will install the Rust compiler and Cargo.

<b>Feeds to Readeck</b> uses [rustls][rustls] for HTTPS requests,
so no system TLS library (e.g. OpenSSL) is required.

## Usage

### Installation

In a terminal or command prompt,
run the following command:

    $ cargo install feeds-to-readeck

If you want to install an update, run:

    $ cargo install --force feeds-to-readeck

[rustls]: https://github.com/rustls/rustls
[rust]: https://www.rust-lang.org/

### Configuration

<b>Feeds to Readeck</b> uses a file to store your configuration
(the list of feeds to monitor and which entries have already been processed).
You must specify a file name as a command-line argument
when you call the program;
there's no default file name.

First, you must create your configuration file:

    $ feeds-to-readeck ~/feeds-to-readeck.yaml init

> `~/feeds-to-readeck.yaml` is just an example,
> you can use any file name you want!

<b>Feeds to Readeck</b> talks to your Readeck instance
using [token authentication][readeck-token-auth].
You must provide the following environment variables
whenever you run `feeds-to-readeck`:

- `READECK_URL`: the base URL of your Readeck instance
  (for example, `https://readeck.example.com/`).
- `READECK_AUTH_TOKEN`: an API token created in your Readeck account settings,
  with at least permission to create bookmarks.

[readeck-token-auth]: https://readeck.org/en/docs/api#overview--token-authentication

### Adding feeds

Once the above configuration steps are done,
you're ready to add feeds.
Use the `add` subcommand to add a feed:

    $ feeds-to-readeck ~/feeds-to-readeck.yaml add https://xkcd.com/atom.xml

This will download the feed
and mark all current entries as "processed"
without sending them to Readeck.
If you would like all current entries to be sent to Readeck,
pass the `--unread` flag:

    $ READECK_URL=https://readeck.example.com/ READECK_AUTH_TOKEN=xxx \
        feeds-to-readeck ~/feeds-to-readeck.yaml add --unread https://xkcd.com/atom.xml

Repeat this for every feed you'd like <b>Feeds to Readeck</b> to monitor.

### Sending new entries to Readeck

Call `feeds-to-readeck` without a subcommand
to have it download your feeds
and send new entries to Readeck.

    $ READECK_URL=https://readeck.example.com/ READECK_AUTH_TOKEN=xxx \
        feeds-to-readeck ~/feeds-to-readeck.yaml

Once an entry has been sent to Readeck,
<b>Feeds to Readeck</b> marks it as "processed"
and will not send it again.

### Assigning tags to feeds

You can assign tags to feeds.
When a new entry is pushed to Readeck,
it will be assigned the tags that were set
on the feed the entry comes from
(sent as bookmark labels).

To do this, pass the `--tags` option
to the `add` subcommand.
You can do this while adding a new feed
or for an existing feed
(then it will *replace* the list of tags for that feed).
The `--tags` option is followed by a comma-separated list of tags.

    $ feeds-to-readeck ~/feeds-to-readeck.yaml add --tags comics,xkcd https://xkcd.com/atom.xml

### Scheduling

<b>Feeds to Readeck</b> doesn't have any built-in scheduling mechanisms.
You should use an existing task scheduler
to run the `feeds-to-readeck` program periodically.

If you are using Linux with systemd,
you can set up a systemd timer
for your systemd user instance.
See the example unit files in the `systemd-examples` directory.

### Removing feeds

Use the `remove` subcommand to remove a feed:

    $ feeds-to-readeck ~/feeds-to-readeck.yaml remove https://xkcd.com/atom.xml

## Compiling from source

To build the project, just run:

    $ cargo build

from the project's directory.
This will download and compile
all of the project's Rust dependencies automatically.

## Contributing

See [CONTRIBUTING][contributing].

[contributing]: CONTRIBUTING.md
