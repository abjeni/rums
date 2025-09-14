# *Rums*

(Very unstable lib!)

Trying to bring the same features of [Gorums](https://github.com/relab/gorums) to rust.
The library currently only supports sending tcp requests to multiple servers, receiving responses in the order they are received through an async stream.
There is a protobuf generator for prost-build, streaming rpc calls are not implemented.

check out the examples.

todo list:
1. finish the todo list
2. supply sender/receiver id/address
3. streaming tcp requests (proto specific?)
4. per node tcp data
5. get rid of unneccesary trait constraints

This is my first project, so I ended up using the Send trait, the 'static lifetime as well as the Vec type along with cloning where it is not needed, in order to make the project compile.
I am open to all kinds of suggestions.