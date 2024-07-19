# Jira CLI

Jira CLI is a command-line client for managing Jira-like issues, epics, and projects using a local database. It allows you to manage your tasks directly from your terminal, providing a streamlined and efficient workflow for developers and project managers.

## Features

- Create, update, and delete issues and epics.
- View and manage project details.
- Manage issue statuses.
- Store data locally using a JSON file.

## Installation

To install Jira CLI, clone the repository and build the project using Cargo:

```sh
git clone https://github.com/dobleuber/jira-cli.git
cd jira-cli
cargo build --release
```

Move the compiled binary to a directory in your PATH:

```sh
cp target/release/jira-cli /usr/local/bin/
```

## Configuration

Before using Jira CLI, you need to configure the path to your local database file. Create a configuration file at `~/.jira-cli/config.toml`:

```toml
[database]
file_path = "/path/to/your/database.json"
```

## Usage

Here's a brief overview of the available commands:

### Create an Epic

```sh
jira-cli create-epic --name "New Epic" --description "Description of the new epic"
```

### Create a Story

```sh
jira-cli create-story --epic-id 1 --name "New Story" --description "Description of the new story"
```

### Update Issue Status

```sh
jira-cli update-status --issue-id 2 --status "In Progress"
```

### Delete an Epic

```sh
jira-cli delete-epic --epic-id 1
```

### Delete a Story

```sh
jira-cli delete-story --epic-id 1 --story-id 2
```

### List Issues in a Project

```sh
jira-cli list-issues --project "PROJECT_KEY"
```

### Search Issues

```sh
jira-cli search-issues --query "assignee = 'John Doe' AND status = 'Open'"
```

## Contributing

We welcome contributions to the Jira CLI project! Please fork the repository and submit pull requests for any features, bug fixes, or improvements.

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please open an issue on GitHub or contact us at [dobleuber@gmail.com](mailto:dobleuber@gmail.com).
