# Macros

## Para ver as macros expandidas

Instale o cargo-expand
```sh
cargo install cargo-expand 
```


Use o comando:
```sh
cargo expand
```

Caso seja necessário instale a versão do rust nightly:

Installing Rust nightly
With rustup, the tool we installed in Chapter 1, Basics of Rust, it is very easy to install nightly:

```sh
rustup default nightly
```
Running this command will install the nightly version of the tools (cargo, rustc, and so on). Also, it will switch the corresponding commands to use the nightly version.

If you want to go back to the stable version, issue the following command:

```sh
rustup default stable
```

The nightly version is updated very frequently, so you might want to update it every week or more often. To do so, you need to run this command:

rustup update
This will also update the stable version if a new version was released (one stable version is released every 6 weeks).

Now that we are using Rust nightly, we're ready to create ...