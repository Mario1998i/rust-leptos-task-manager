# Rust + Leptos Task Manager

A simple web application for managing daily tasks, built with Rust and Leptos.

## About the Project

This project is a task manager that allows users to create tasks, mark them as completed, and delete them.

It was developed to explore Rust web development and learn how to build an interactive web interface using Leptos.

## What Can You Do?

- **Add tasks:** create new tasks using the text field.
- **Complete tasks:** mark tasks as completed using the checkbox.
- **Delete tasks:** remove tasks from the list.
- **Responsive interface:** use the application on different screen sizes.

Empty tasks cannot be added. Completed tasks are displayed with a strikethrough.

## Technologies Used

- **Rust:** handles the application's logic.
- **Leptos:** builds the interactive user interface.
- **HTML:** provides the basic page structure.
- **CSS:** defines the layout, colors, spacing, and appearance.
- **Trunk:** prepares the application and runs the local development server.
- **WebAssembly (Wasm):** allows the compiled Rust application to run in the browser.

## How to Run the Project

### 1. Install the required tools

To run the project, you need Rust, the WebAssembly compilation target, and Trunk.

### 2. Download the project

Open a terminal and run:

```bash
git clone https://github.com/Mario1998i/rust-leptos-task-manager.git
cd rust-leptos-task-manager
```

### 3. Start the application

Run the following command:

```bash
trunk serve
```

Open the local address displayed in the terminal, usually `http://localhost:8080`.

## Current Limitations

Tasks are stored in memory. This means they are lost when the page is refreshed or the application is closed.
