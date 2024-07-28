# Rust-Apps
This repository is for my rust practice where I'll put some rust application i built for practice.

## App1: Ip-Sniffer
This application allows user to find out open port on given ip-address.

Note: This app accepts cli-argument '-j' (number of threads you want to run). Since unsuccessful connection try takes around 2.09 seconds on my pc, I suuggest choosing threads over 10000.

Usage: ```cargo run -- -j 10000 <ip-address>```
