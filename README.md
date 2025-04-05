# GitHub File Monitor

A Rust application that monitors changes in specified files across GitHub repositories and provides notifications when changes are detected.

## Features

- Monitor multiple GitHub repositories simultaneously
- Track specific files within each repository
- Detect file changes using SHA-256 hashing
- Maintain state between runs
- Configurable notification system

## Installation

1. Clone the repository
2. Ensure you have Rust installed
3. Build the project:
```bash
make build
```

## Configuration

Create a `config.yaml` file in the project root with the following structure:

```yaml
repositories:
  - owner: repository-owner
    repo: repository-name
    token: github-access-token
    branch: main
    files:
      - path/to/file1
      - path/to/file2
state_file: state.yaml
notification_command: "your-notification-command"
```

### Configuration Fields:
- `repositories`: List of GitHub repositories to monitor
- `owner`: Repository owner/organization
- `repo`: Repository name
- `token`: GitHub access token
- `branch`: Branch to monitor
- `files`: List of files to track
- `state_file`: Location to store the state
- `notification_command`: Command to execute when changes are detected

## Usage

Run the application:

```bash
cargo run
```

The application will:
1. Read the configuration from `config.yaml`
2. Check specified files for changes
3. Update the state file when changes are detected
4. Execute the notification command when changes occur

## Dependencies

- `reqwest`: HTTP client for GitHub API requests
- `serde`: Serialization/deserialization of YAML
- `sha2`: SHA-256 hashing functionality
- `tempfile`: Temporary file handling for tests

## Testing

Run the test suite:

```bash
make test
```

The test suite includes:
- Configuration deserialization tests
- State serialization tests
- File hash retrieval tests
- Change detection tests
