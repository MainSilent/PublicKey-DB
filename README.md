# ECDSA PublicKey Database

Fast and Efficient database made specifically for ecdsa public keys to test different brute force attacks


### Why not using other databases or a simple file?

1- It uses unix socket instead of TCP/IP

2- Since keys are just a number when adding it will sort them automatically

3- By using binary search in sorted data, You will get the result much faster

## Build

```
cargo build
```

## Setup

Configure `config.json` by adding the storage path and socket path

```json
{
    "socket": "/tmp/pubdb.sock",
    "storage": "/home/main/pubkeys/"
}
```

## How to use

`"add {key}"` | Add a new public key (x-value 64 char length)

`"find {key}"` | Search if a public key exists (x-value 64 char length)

All the commands will return `1` in success or an error in failure
