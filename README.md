
# *Rums*

(Very unstable lib!)

Trying to bring the same features of [Gorums](https://github.com/relab/gorums) to rust.
The library currently only supports sending tcp requests to multiple servers, receiving responses in the order they are received through a blocking iterator.
There is a protobuf generator for prost-build, streaming rpc calls are not implemented.

check out the examples.

This branch uses mio instead of tokio which makes rums usable outside of the tokio async runtime.
The structure of the files, structs, traits and names is very messy but after gatting the examples to work I have decided that it is better to focus on tokio.