# Plan

## Overview
A CLI tool that scans JavaScript and TypeScript files for adherence to standard practices, code cleanliness, and quality rules. Designed to be added to `lint-staged` and `husky` pipelines.

Built with **Rust** using the `oxc` parser for blazing-fast AST parsing.

## Features
- Scan JS/TS files against a comprehensive set of rules covering standard practices, code cleanliness, and quality
- Output violations with file location and rule description to the terminal
- Optionally write results to a log file
- Designed for integration with `lint-staged` and `husky`
- Single-binary distribution — no Node.js or `node_modules` required

## Rules
Include all possible rules covering:
- Code quality
- Best practices
- Clean code
- React patterns
- TypeScript strictness
- Security
- Performance

## Output
- Location (file path + line number) of each violation
- The specific rule that was violated
- Terminal output by default
- Optional log file export
