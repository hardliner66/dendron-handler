# Dendron Handler
Small helper for opening dendron files locally in vscode through links.

## Install
### Pre-built binaries
You can get the pre-built binaries from the [releases](https://github.com/hardliner66/dendron-handler/releases/latest)

### From source
`cargo install --git https://github.com/hardliner66/dendron_handler`

## Usage
In order for the handler to work with links, you have to register it first.
To register the handler on windows, just run the tool and it will do it automatically.

On other systems you have to do it manually, as I don't know how to automate that (yet). PRs are welcome.

After the handler is registered, clicking a link with the right format should open your dendron
workspace and the specified files.

To help creating links, I created a simple [helper extension](https://github.com/hardliner66/dendron-handler-extension).

## URL Format
```html
dendron://<vault>/<relative_path[:line[:character]]>
```

- `<vault>`: Name of the Dendron vault (e.g., `my-notes`). An empty vault name is treated as `"default"`.
- `<relative_path>`: Path to a file, relative to the root of the vault
- `:line` *(optional)*: Line number
- `:character` *(optional)*: Character offset in the line

### Examples
```sh
dendron://default/notes/todo.md
dendron:///notes/todo.md                  # Equivalent to above (empty vault = "default")
dendron://my-vault/test/some_doc.md:42
dendron://my-vault/test/some_doc.md:42:13
```

## Config
### Path
Dendron handler looks for a file named `dendron-handler.json` in the following directories:

| Platform | Value                                 | Example                                  |
| -------- | ------------------------------------- | ---------------------------------------- |
| Linux    | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                      |
| macOS    | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
| Windows  | `{FOLDERID_LocalAppData}`             | C:\Users\Alice\AppData\Local             |

### Fields
### `default_vault` (optional)

- **Type**: `String`
- **Description**: The name of the default vault to use when none is specified.
- **Default**: If not set, `"default"` is used.

### `vaults` (optional)
- **Type**: Map<String, Path>
- **Description**: A mapping from vault names to their root paths on the local file system.

#### Example
```json
{
    "default_vault": "my-vault",
    "vaults": {
        "my-vault": "c:/my_vault"
    }
}
```