

[Cargo Watch](https://github.com/watchexec/cargo-watch)

Para rodar os testes automaticamente quando vc alterar um arquivo instale o cargo watch:
```sh
cargo install cargo-watch
```

Depois basta rodar na base do projeto projeto:
```sh
cargo watch -x 'test -- --nocapture'
```
