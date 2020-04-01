# Graph Gen
Graph Gen is a conveniency tool (and library _crate_) that lets you generate
pseudo-random graphs based on the Erdos-Renyi G(n,p) model.

This tool is mainly designed to be simple and reasonably fast. Its purpose was
for me to use random input graphs in my research experiments. If you like it
and/or find it useful, that's a bonus.

## Options
Graph Gen comes with several options:
+ It lets you build graphs that allow/not allow self loops (`-l` flag)
+ It lets you build directed graph if that is what you need (`-d` flag)
+ It lets you specify weights that can be used as random labels for the edges of your graph (`-w` option).
+ It lets you output your graph either in DIMACS or in GraphViz format.

All necessary info should be available with the built in help.

## Help
```
graph_gen 0.1.0
Convenience tool to generate pseudo random graphs

USAGE:
    graph_gen [FLAGS] [OPTIONS] --nb_vertices <nb_vertices> --probability <probability>

FLAGS:
    -d, --digraph    If set, the generated graph will be a digraph
    -h, --help       Prints help information
    -l, --loops      If set, self loops are allowed in the generated graph
    -V, --version    Prints version information

OPTIONS:
    -n, --nb_vertices <nb_vertices>    The number of vertices in the generated graph
    -o, --output <output>              The output language (defaults to dimacs)
    -p, --probability <probability>    The likelihood of any edge to be picked
    -w, --weights <weights>...         Optional weight candidates

```

## Build
Graph Gen was written in Rust. As such, it is compiled with the `cargo` tool.
So `cargo build --release` will produce the release binary in the `target` folder.


