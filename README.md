# download-service

**A gRPC micro service for downloading YouTube videos**

---

Provides all the necessary APIs to download different different resources from YouTube.

## Cloning

To clone this repository:

    git clone --recurse-submodules -j8 ssh://git@gitea.girgis.me:32822/tube-recorder/download.git

To make sure `master` branch is checked out:

    git submodule foreach "(git checkout master; git pull)&"

## Usage

To start the server:

    cargo run --bin server

## Development

A client is provided as a testing helper:

    cargo run --bin client

## Change Log

A complete history of the change log can be found [here](./ChangeLog.md)

## TODO

An up-to-date list of development aspirations can be found [here](./TODO.md)
