# xmpp-gui

### _Mohammed Alsaid_
### _CS510: Rust Programming_

## Idea
This is a XMPP GUI client proejct for `CS510/410: Rust Programming`. It uses [xmpp-library](https://gitlab.com/xmpp-rs/xmpp-rs.git) to build a GUI client. Through the client, users could connect, join rooms and send messages.

## How it went (challanges)

The biggest challange I faced in this project was trying to use a not very-well documented crate to achieve my objective. Moreover, I initially had issues using some functions from `xmpp`. The cause was due to pulling directly from crates.io instead of the git repo.

Another issue was caused by my design decision with regards to the `Iced` create. The library provides an `iced::Application` trait that is the main entrypoint point of Iced. There's also the `iced::Sandbox` trait, which provides a simple interface for a GUI application. Using the `Sandbox` for an xmpp client proved to be challanging as a Sandboxed application cannot run asynchronous actions.

## what didn't work
The fact that the GUI application is not able to run asynchronous actions, limits the functionalities that the user can have. It also limits me to what I can use from `xmpp` crate. Moreover, I wasn't entierly sure how I can go about testing a GUI application in Rust. Building a simple state handler seemed to be a solution for the issue. It provides an API that I can test and a way to deal with the limitation of `iced`. 

## What next:
I think a change in the traited used from `iced` is necessary. A networking application needs to be asynchronous if we want it to be scalable. I don't see how supporting the feature of sending and recieving messaging would be feasible in the current condition. 


# Build and test:
* Build: simply use `cargo run` to run the application. 
* Test: run `cargo test` to run the tests. 

# Servers used for testing:
I used `prosody` to run a local XMPP server for testing. Moreover, I used public servers linked below to test with public servers.

# Resources:
[1]. Run and configure an xmpp [server](https://xmpp.org/software/servers.html)

[2]. Use public servers from this [list](https://xmpp.net/directory.php)
* only used `jabber.de` to register clients for testing.

[3]. iced [crate](https://docs.rs/iced/0.1.0-beta/iced/index.html)

[5]. xmpp-rs [crate](https://gitlab.com/xmpp-rs/xmpp-rs)