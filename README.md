# kudu-rs

bind kudu version 1.7.1

Experimental Rust bindings for Apache Kudu (incubating). Not feature complete.

[Documentation](https://danburkert.github.io/kudu-rs/kudu/index.html)


# build
do not use rust stable version to build. 

because prost-derive = "0.4" at stable rust version got error:

```
^^^^^^^^^^^ multiple `intersperse` found
```

so,try with 


```bash
cargo +nightly build
```

You will need to add `kudu-master` and `kudu-tserver` to the $PATH, or set
`$KUDU_HOME` to your checkout of the kudu repository, and build Kudu in
`$KUDU_HOME/build/latest` (see the Kudu [build from source
instructions](http://getkudu.io/docs/installation.html#_build_from_source)).

```bash
env KUDU_HOME=<path-to-kudu-checkout> cargo build
```
