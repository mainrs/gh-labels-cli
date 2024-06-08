> [!NOTE]
> The official GitHub CLI now provides similar functionality via the `gh label` subcommand. The bulk import of labels is supported via the `gh label clone` command.

# gh-labels-cli

> A tool for managing your GitHub labels.

## Use-cases

I personally use the CLI for creating a standard set of labels when initializing a new repository on GitHub. `gh-labels-cli` can integrate into the [official GitHub CLI](https://cli.github.com) to make it easier to manage repository labels.

## Installation

`gh-labels-cli` can be installed via homebrew:

```console
$ brew install mainrs/tap/gh-labels-cli
```

You can also build it from source using `cargo`:

```console
$ cargo install gh-labels-cli --locked
```

An AUR package is in the works :)

## Usage

The CLI can be used as either a standalone binary by directly invoking it via `gh-labels` or you can register aliases for the official GitHub CLI (`gh`). To register the aliases, run `gh-labels integration install`.

The CLI needs a personal access token with appropiate `public_repo` or `repo` scope, depending on whether you want it to work on private repositories as well. The token can be passed to the CLI using a CLI argument or via the environment variable `GH_LABELS_TOKEN`.

> **Note:** Some poeple may wish to re-use a singleton token across multiple CLIs. The CLIs I've stumbled across often use the `GITHUB_TOKEN` environment variable. This is also supported. The order in which the token is tried to be read from is `CLI argument` > `GH_LABELS_TOKEN` > `GITHUB_TOKEN`.

The CLI operates on repositories. Those can either be directly supplied via an argument in the form `owner/repo` or by running the CLI inside of an existing git repository with an upstream named `origin` pointing to `github.com`.

For more information, take a closer look at the help.

## Commands

### Config

Used to query the configuration file path, content or to edit the configuration inside your terminal.

### Integration

Used to install and uninstall the `labels` and `new` aliases for the `gh` CLI. `gh labels` can be used to create a single label. `gh new` generates a new repository and automatically applies your own [label definitions file](#label-definitions-file) to it.

### Api

Commands related to actual GitHub API calls.

#### Create

Creates a single label inside a repository with the given values.

#### Update

Bulk-create labels and update existing ones inside of repositories. You have to supply a [label definition file](#label-definitions-file) for the command to work. The file can be supplied via the `-f,--file` argument or by putting the file inside the directory returned via the `gh-labels config --path`. The file has to be named `labels.json` when using the second option.

## Label definitions file

A label definitions file is a manually maintained file containing all the labels you want to apply to a repository. It's a JSON file with the following format:

```json
{
  "labels": [
    {
      "name": "type: bug",
      "color": "431232",
      "description": "A programming error"
    }
  ]
}
```

> **Note:** The description field is optional.

My own label definition file can be found [here](https://gist.github.com/mainrs/1fd1bb7f21c8d9170e69f52aa38c3201).

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

