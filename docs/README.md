# Egiraffe Documentation

This directory contains the documentation for Egiraffe.

## Debug vs. Production Build
If you want to make a production build, pass `--features prod` to cargo. This will change the default configuration and add some safety assertions.

## Configuration
See conf.rs

The Configuration is determined by:
* defaults (different defaults for dev and prod)+
* a toml file
* Environment variables (EGIRAFFE_-Prefix)

In case of a Production Build, there are some assertions for safety in the code.