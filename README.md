# Gitter - Git Automation Tool

## About
The Git Automation Tool simplifies common Git operations such as merging feature branches, pulling updates, and pushing changes, eliminating the need for manual commands or GUIs.

## How it works
Gitter uses *.txt* files inside *gitter* directory to store git commands and execute them. Each file is one task and each line inside task is one git command. Tasks can be called anything, for example `commit.txt` and it will be run with `gitter run commit`.

## Getting started
Download Gitter executable or download source code and build it with `cargo build --release` <a href="https://www.rust-lang.org/learn/get-started">rust - get started</a>

## Commands

<em>Note: gitter should be added to PATH and run from root of repository where '.git' folder is located</em>

**init** - Creates new gitter folder structure with files.

**set** - Configure settings for the tool.

**show** - Displays settings values.

**run** - Runs automated git task.

### Command Details

**`init`**
- This command does not have flags.
- Running this will automatically create <em>gitter</em> directory inside <em>.git</em> with <em>settings.txt</em> file

**`set [-k | --key] <key> [-v --value] <value>`**
- Configure specific settings for the tool or add new entry to <em>settings.txt</em>
- Example: `gitter set --key branch_name --value main`

**`show`**
- [-a|--all] Displays all values from settings.
- [-k|--key] Displays value of a key. 
- Examples: `gitter show --all` | `gitter show --key branch_name`

**`run <task> <args>`**
- `<task>` is name of the '.txt' file in gitter folder that should be run.
- `<args>` are key value pairs that you want to inject into you placeholders in command
- Example `gitter run commit mess="Commit message"`

### Examples
1. Start by calling `gitter init` inside root of a repository
2. Inside `.git/gitter` create task called `commit.txt`
3. Inside `commit.txt` write git commands <br>`add .`<br>`commit -m {{m}}`<br>`push origin my_branch`
4. Call `gitter run commit m="Commit message"` to run `commit.txt` task

**Injection examples**
-`${<key>}` placeholder is used to inject value from settings
-`{{<key>}}` is used to inject value from passed arguments
- Example: <br>`commit {{m}}`<br>`checkout ${branch}`
