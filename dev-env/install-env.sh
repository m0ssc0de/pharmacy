#!/bin/bash

set -eux

yum -y install  https://centos7.iuscommunity.org/ius-release.rpm

yum install -y gcc cmake sqlite-devel git2u-all

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source $HOME/.cargo/env

cargo install cargo-web

cargo install diesel_cli --no-default-features --features sqlite