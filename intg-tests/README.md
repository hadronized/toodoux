# Integration environment

This directory holds a bunch of data you can use to test a change you made / are making. The way you should be using
this is — at the root of the repository, for instance:

```sh
cargo run -- -c ./intg-tests <regular options>
```

For instance, to list all the tasks in the data set:

```sh
cargo run -- -c ./intg-tests ls --all
```
