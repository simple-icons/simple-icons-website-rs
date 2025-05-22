# simple-icons-website-tests

Integration unit tests for the Simple Icons website.

This crate must be used to tests things that are common to several different parts of the repository. For example, to ensure that the file _USAGE.md_ is up to date. Don't use it to test individual crates (add them to those crates instead) nor end-to-end tests (add them to the _tests/end2end_ crate instead).
