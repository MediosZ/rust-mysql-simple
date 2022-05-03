# Mysql wasi

## Prerequisits

1. install mysql, create a database named wasm.
2. install target `wasm32-wasi`.

## Demo

1. query data from mysql
    `cargo r --bin query_test`
2. insert data to mysql 
    `cargo r --bin insert_test`


## Notes

1. I disable `tls` for now.
2. I use a forked [wasmedge_wasi_socket](https://github.com/MediosZ/wasmedge_wasi_socket/tree/mysql).