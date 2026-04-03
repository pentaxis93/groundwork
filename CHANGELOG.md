# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Fixed

- Removed redundant `schema` fields from `groundwork.toml` artifact type declarations so `runa init --methodology groundwork.toml` can derive schema paths by convention and parse the manifest successfully.
- Declared all execution-phase protocols in `groundwork.toml` as `scoped = true` so Groundwork remains compatible with runa's scoped artifact validation for `work_unit`-threaded outputs.
