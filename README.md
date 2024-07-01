# Gitter - Git Automation Tool

## Usage
The Git Automation Tool simplifies common Git operations such as merging feature branches, pulling updates, and pushing changes, eliminating the need for manual commands or GUIs.

## Commands

**set** - Configure settings for the tool.

**commit** - Perform Git operations including committing changes, pulling updates, merging branches, and pushing commits. All options are optional as defaults can be retrieved from settings.

### Command Details

**set [--key] <key> [--value] <value>**
- Configure specific settings for the tool.
- Example: `set --key branch_name --value main`

**commit [-m|--message] <message> [-l|--local] <local_branch> [-f|--feature] <feature_branch> [-p|--path] <repository_path> [--push] [--pull]**
- Commit changes with a specified message, pull and merge from a feature branch into a local branch, and optionally push changes. All options are optional and can be pulled from settings.
- Example: `commit -m "Fix: Issue #123" -l main -f feature-branch -p /path/to/repository --pull --pull`
- Example with values in settings `commit -m "Fix: Issue #123 --pull --push"`

---

This documentation provides clear instructions and examples for using the `set` and `commit` commands in the Git Automation Tool, with all options for the `commit` command being optional.

