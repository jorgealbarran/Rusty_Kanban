'''# Rusty Kanban

A simple command-line Kanban board application written in Rust.

## Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/Rusty_Kanban.git
    cd Rusty_Kanban
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Add the executable to your PATH:**

    For convenience, you can add the compiled binary to a directory in your system's `PATH`.

    ```bash
    sudo cp target/release/rusty_kanban /usr/local/bin/
    ```

## Usage

`rusty_kanban` is a simple command-line tool to manage your tasks. It supports adding, listing, updating, and deleting tasks.

### Commands

*   `add <task>`: Add a new task to the to-do list.
*   `list`: List all tasks, categorized by status (To Do, Doing, Done).
*   `doing <position(s)>`: Mark one or more tasks as 'doing'.
*   `done <position(s)>`: Mark one or more tasks as 'done'.
*   `delete <position(s)>`: Delete one or more tasks.

### Options

*   `-f, --file <path>`: Specify a custom path to the journal file. Defaults to `~/.config/Rust-Kanban/rust-kanban.json`.

### Examples

1.  **Add a new task:**

    ```bash
    rusty_kanban add "Implement the core logic"
    ```

2.  **List all tasks:**

    ```bash
    rusty_kanban list
    ```

3.  **Mark a task as 'doing':**

    ```bash
    rusty_kanban doing 1
    ```

4.  **Mark multiple tasks as 'done':**

    ```bash
    rusty_kanban done 1 2
    ```

5.  **Delete a task:**

    ```bash
    rusty_kanban delete 3
    ```

6.  **Use a custom journal file:**

    ```bash
    rusty_kanban -f /path/to/your/kanban.json list
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
''