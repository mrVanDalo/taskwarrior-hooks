
## AutoTagger

> under development

automatic adds tags by parsing the description.

### How to run

```sh
cargo run ./test-data/AutoTagger/valid-tag-map.json
```

### How to use

#### on-add 

To use for on-add 

```sh
AutoTagger <path/to/tag-map.json>
```

#### on-modify

to use for on-modify (just drop the original line)

```sh
read line
AutoTagger <path/to/tag-map.json>
```

# Tests

```
cargo test
```

will test everything
