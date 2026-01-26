# Telegram Send File Action

This GitHub Action allows you to send files from your GitHub repository to
Telegram. Perfect for sharing build artifacts, logs, or any files directly
through Telegram messages.

## Inputs

### `token`

**Required** Bot API token.

### `chat_ids`

**Required** Chat IDs of the recepients.

### `files`

**Required** The list of file paths to upload, one file per line.

### `body`

**Optional** The message to send along with the files.

### `api_url`

**Optional** Custom Bot API URL.

### `pin`

**Optional** Pin chat message.

## Outputs

None

## Example usage

This example demonstrates how to use the Telegram File Sender action with the
required inputs and environment variables.

Create a `.github/workflows/tg-send-files.yml` (or add to your existing
workflow file):

```yaml
name: Send files to Telegram

on: [push]

jobs:
  send-files-to-telegram:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Send files to Telegram
      uses: ardocrat/tg-send-file-action
      with:
        token: ${{ secrets.BOT_TOKEN }}
        chat_ids: |
          -1234567890
          -0987654321
        files: |
          /path/to/file1
          /path/to/file2  
        body: 'Here are your files!'
        pin: true
```

Make sure the actual values of `chat-id` and `files` inputs. Also, remember to
set `token` in your repository's secrets.

This README provides a basic overview of your GitHub Action, how to use it, and
how to configure it with necessary environment variables. Adjust the content
according to your actual action's repository, inputs, and needs.
