# Bin Files
Create a cli tool where the logic is split out nicely into separate functions and files. 

<br/>

## Backstory
Imagine you want to make a cli tool that ultimately builds into a single binary executable file but depends on lots of different functions.

The idea is that this code can serve as a proof-of-concept for a medium to large-scale codebase all contained within a single binary project.

<br/>

## The Exercise
The exercise is to create three functions in three separate files, all called from `main` which should print the total sum of the three numbers returned by `fn1`, `fn2`, and `fn3`.

1) The first function should be named `fn1` and should live inside a module named `mod1`. This function should just return the number 1. 

2) The second function should be named `fn2` and should live inside a module named `mod2`. This module should import `f1` from `mod1`, and `fn2` should return 2 times the result of calling `fn1`.

3) The third function should be named `fn3` and should live inside a module named `mod3` (which itself may or may not live inside of other modules). This file should be inside of a subfolder and not next to the main.rs file, and it should return the number 3 when called.

<br/>

## Tests
The unit tests for fn1 and fn3 should check that they return the proper values. Experiment with returning a mocked value of fn1 when calling the fn2 unit test.

As an integration test, verify that the grand total of 6 is printed to the console.

<br/>

## Skills Practiced

- Creating helper functions and modules

- Importing a helper function and module in main.rs

- Importing a helper function and module into another helper function and module

- Importing a helper function and module from a subfolder

<br/>

https://code.visualstudio.com/docs/editor/settings-sync

Codespaces - Provides a cloud IDE through Visual Studio Code. Offers a normalized environment.

Dev containers - Configuration files that define the Codespaces environment.

Devcontainer.json - The main dev container config file that sets up tools, extensions, etc.

Dockerfile - Optional file to customize the OS and install system packages.

Rebuild container - Rebuilds the Codespaces environment with your new config files.

Quotas - Usage limits for Codespaces, including compute time and storage.

RustUp the toolchain installer

Binary application/package: Executables generated from Rust source files, typically containing a main function.

Library: A collection of Rust modules providing functionality meant to be shared among multiple projects.

Cargo.toml: A configuration file read by Cargo, listing metadata (e.g., name, version) and dependencies required by the package.

Shadowing: Reassigning a variable to a new value while preserving its original binding, enabling changes to its type or scope.

Control Flow: Conditional execution paths based on evaluation of logical expressions, including if, else, and else if clauses.

Scope: A region within source code where names (e.g., functions, variables) are accessible; determined by enclosing braces ({}) or indentation levels.

Semicolons: Terminators denoting statement boundaries, required in most cases except inside blocks, expressions, and macros.


Note: DO NOT use the "play" or "run" buttons on Visual Studio Code. Use the integrated terminal. Rust and Cargo are in the /bin/cargo/bin/ path

Instructions:
Open the lab which will place you in the examples directory of the Week 2 GitHub Repository. You will create your project here.

Open a terminal/command prompt and ensure that you have Cargo installed by running the command `cargo --version`.  DO NOT use the "play" or "run" buttons. Rust and Cargo are in the /bin/cargo/bin/ path

Run the command `cargo new my_project` to create a new Rust project named "my_project". Replace "my_project" with your preferred project name.

Navigate into the newly created project directory by running the command `cd my_project` (replace "my_project" with your project name).

Explore the contents of the project directory. You will find files such as `Cargo.toml` (the manifest file that describes the project) and a `src` directory (which contains the source code).

Open the project directory in your preferred code editor.

Modify the automatically generated `src/main.rs` file with your desired Rust code. You can start with a simple "Hello, World!" program or any other code snippet.

Save your changes to the `main.rs` file.

In the terminal/command prompt, run the command `cargo build` to build the project. Cargo will compile your code and generate the necessary binaries.

After a successful build, run the command `cargo run` to execute your Rust program.
