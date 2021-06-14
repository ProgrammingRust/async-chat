# Asynchronous chat client and server from _Programming Rust_

This repository contains the source code for the chat client and server written
using the Rust language's asynchronous programming features. It is described in
Chapter 20 of the book _Programming Rust_, by Blandy, Orendorff, and Tindall
(ISBN 978-1-492-05259-3).

To run the server, type:

    $ cargo run --release --bin server -- localhost:8088

To run the client, type:

    $ cargo run --release --bin client -- localhost:8088

The client supports only two commands:

- <code>join <var>group</var></code> - Join the group named <var>group</var>. If
    that group does not exist, it is created. The name of the group must not
    contain any spaces.

- <code>post <var>group</var> <var>message</var></code> - Post
    <var>message</var> to the chat group named <var>group</var>. The group name
    must not contain any spaces, but the message can.

There is no command to leave a group. There is no concept of a user name. To
exit the client, hit ctrl-D on Linux or macOS, or ctrl-Z on Windows.

An example client session:

    $ cargo run --release --bin client -- localhost:8088
        Finished release [optimized] target(s) in 0.04s
         Running `/home/jimb/rust/book/tests/chapters/asynchronous/target/release/client 'localhost:8088'`
    Commands:
    join GROUP
    post GROUP MESSAGE...
    Type Control-D (on Unix) or Control-Z (on Windows) to close the connection.
    join dogs
    post dogs I love dogs!
    message posted to dogs: I love dogs!
    message posted to dogs: Whaddya know, I do too!
    message posted to dogs: Hello, dog lovers!
    post dogs Hi!
    message posted to dogs: Hi!
    ctrl-D
    $
