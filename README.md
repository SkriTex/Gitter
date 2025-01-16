# Gitter - Git Automation Tool

## Usage
The Git Automation Tool simplifies common Git operations such as merging feature branches, pulling updates, and pushing changes, eliminating the need for manual commands or GUIs.

## Commands

<em>Note: gitter should be added to PATH and run from root of repository where '.git' folder is located</em>

**init** - Creates new gitter folder structure with files.

**set** - Configure settings for the tool.

**show** - Displays settings values.

**run** - Runs automated git task.

### Command Details

**init**
- This command does not have flags.

**set [-k | --key] <key> [-v --value] <value>**
- Configure specific settings for the tool.
- Example: `set --key branch_name --value main`

**show**
- [-a|--all] Displays all values from settings.
- [-k|--key] <key> Displays value of key.
- Examples: `show --all` | `show --key branch_name`

**run <task> <args>**
- <task> is name of the '.txt' file in gitter folder that should be run.
- <args> are key value pairs that you want to inject into you placeholders in command
- Example `run myTask mess="Commit message"`

**Injection examples**
-${<key>} placeholder is used to inject value from settings
-{{<key>}} is used to inject value from passed arguments
- Example: <em>`commit {{m}}`<br>`checkout ${branch}`</em>
- Gitter call example: <em>`gitter run task m="Commit message"`</em>

**Task example**
<em>Note that tasks are added manually in gitter directory.
Task can be called anything except settings.
All tasks are written in text files where each line represents one git command.</em>
- Example: <em>myTask.txt</em><br>
`checkout master`<br>`pull origin master`<br>`checkout local`<br>`merge master`
---

This documentation provides clear instructions and examples for using the gitter commands.

