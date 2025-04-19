# Treki CLI
A CLI tool for API testing locally or syncing with your Treki account for advanced testing.

## Features
- Send GET | POST | PUT | PATCH | DELETE requests directly from your terminal.
- View additional information like status code and headers by using verbose mode.
- Send custom body and headers when sending requests.

## Installation

### Linux
Run this command on your terminal, and it will automatically run the installation script and install `treki-cli` on your system.
```sh
curl -sSL https://raw.githubusercontent.com/aether-flux/treki-cli/main/scripts/linux/install.sh | bash
```

### Windows
For Windows users, open PowerShell (preferably in Administrator Mode) and run the following command :
(For PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/aether-flux/treki-cli/main/scripts/windows/install.bat | iex
```
(For CMD)
```cmd
powershell -Command "iwr -useb https://raw.githubusercontent.com/aether-flux/treki-cli/main/scripts/windows/install.bat | iex"
```

## Usage
### Commands
- **get <url>:** Send a GET request.
- **post <url>:** Send a POST request.
- **put <url>:** Send a PUT request.
- **patch <url>:** Send a PATCH request.
- **delete <url>:** Send a DELETE request.

### Options/Flags
- **help or -h or --help :** Display help information about the CLI.
- **-v or --verbose :** Display additional information like status code and headers.
- **-b or --body :** Send a body to the request as a JSON string.
- **-H or --headers :** Send headers to the request like 'key1:val1, key2:val2, ...'.

### Example Usage
```sh
treki-cli post https://jsonplaceholder.typicode.com/posts -b '{"title": "New Post", "body": "Lorem ipsum dolor et smth smth.", "userId": 1}'
```
