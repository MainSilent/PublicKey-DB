# ECDSA PublicKey Database (Beta)

Fast and Efficient database made specifically for ecdsa public keys to test different brute force attacks


### Why not using other databases or a simple file?

1- It uses unix socket instead of TCP/IP

2- The database files only contain the public keys and not any metadata (except the index file), This is why you can store `31250` public keys for only 1MB!

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

Connect to `/tmp/pubdb.sock`

`"add {key}"` | Add a new public key (x-value 64 char length)

`"sort"` | Sort added data

`"find {key}"` | Search if a public key exists (x-value 64 char length)

All the commands will return `1` in success or an error in failure.

To keep the adding operation as fast as possible, The data won't be stored when adding.

Make sure to use `sort` command before searching for a public key.
