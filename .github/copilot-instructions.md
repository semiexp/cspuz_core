# Copilot Instructions for cspuz-core

## Description
This file describes the coding style and conventions for the cspuz-core project. It provides guidelines for writing clean, maintainable, and consistent code across the project.

## Setup

- Ensure you have Rust and C++ installed on your system to work with the cspuz-core project. Most of the code is written in Rust, but the internal SAT solver (Glucose) is implemented in C++.
- Clone the repository, then update all submodules by `git submodule update --init --recursive`.

## Directory Structure

- `cspuz_core`: The core CSP solver implementation in Rust.
- `cspuz_rs`: A wrapper for the cspuz_core library, providing a user-friendly API for Rust applications.
- `cspuz_rs_puzzles`: A collection of puzzle implementations that utilize the cspuz_rs API.
- `cspuz_solver_backend`: The "backend" module for the cspuz-solver2 web application.
- `cspuz_core_python`: A Python wrapper for the cspuz_core library, allowing Python applications to utilize the CSP solver.

## Coding Style Guidelines

- Follow Rust's standard coding conventions, including naming conventions, formatting, and documentation practices.
- Use `format.sh` to format your code before committing. This ensures that all code adheres to the same style.
- Write clear and concise comments to explain the purpose of complex code sections or logic.

## Testing

- Write unit tests for all new features and bug fixes to ensure code quality and prevent regressions.
- Run `cargo test` to execute all tests and ensure they pass before submitting a pull request.
