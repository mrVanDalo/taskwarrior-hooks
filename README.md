# How to install

to install all binaries in `~/.cargo/bin` run.

```
cargo install --path .
```


## auto-tagger 

automatic adds tags by parsing the description.

### How to run

```sh
auto-tagger ./test-data/AutoTagger/valid-tag-map.json
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
read line
~/.cargo/bin/auto-tagger <path/to/tag-map.json>
```

# Tests

```
cargo test
```

will test everything
