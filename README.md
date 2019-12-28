# How to install

to install all binaries in `~/.cargo/bin` run.

```
cargo install --path .
```

# Executables

## auto-tagger 

Automatic adds tags by parsing the description.
The description string will be split in words and the
matching must be exact, no regular expressions supported (yet?).
You have to give configuration file `tag-map.json`,
here is an example :

```
[
  {
    "name": "buy",
    "keywords": ["buy" "shopping" "shop" ]
  },
  {
    "name": "bug",
    "keywords": ["fix", "bug"]
  }
]
```

### How to use

#### on-add 

To use for on-add 

```sh
#!/usr/bin/env bash
~/.cargo/bin/auto-tagger <path/to/tag-map.json>
```

#### on-modify

to use for on-modify (just drop the original line)

```sh
#!/usr/bin/env bash
read original_line
~/.cargo/bin/auto-tagger <path/to/tag-map.json>
```

# Tests

```
cargo test
```

will test everything
