# Customizing Builds with Release Profiles

Cargo has two main profiles:
- dev - good defaults for development
- release - good defaults for release

To build a project with a specific profile, use the `--profile` flag:

```bash
$ cargo build --profile=release
```
Or, for short:
```bash
$ cargo build --release
```

To customize profiles, add a `[profile.<name>]` section to `Cargo.toml`:
    
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls the level of optimization, ranging from 0 to 3.

To create a custom profile, you must specify which profile it inherits from by using the `inherits` directive.

```toml
[profile.my_custom_profile]
inherits = "dev"
opt-level = 1
```