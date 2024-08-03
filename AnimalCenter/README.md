# actix

[actix doc](https://actix.rs/docs/getting-started/)

## git

```shell
{user}/.cargo
create config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

## if git not support

```shell
{user}/.cargo
create config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "http://mirrors.ustc.edu.cn/crates.io-index"
```

## docker

```shell
docker run  -p 8080:8080 -v /.env:/usr/bin/.env --name animal_center -d futugyousuzu/back_animal_center
```

## update package

```shell
cargo install -f cargo-edit
cargo upgrade
```
