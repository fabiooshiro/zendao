# SolZen

Implementação da DAO na rede Solana

Para rodar os testes
```
yarn test
```

Para ver os logs do rust, ou seja, da parte on-chain
```
tail -F .anchor/program-logs/2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv.solzen.log
``` 

## Deploy

Configure o cluster para devnet, testnet ou mainnet-beta em `Anchor.toml`

Confirme que esteja usando a versão stable do Rust
```
rustup default stable
```

```
anchor build
```

```
anchor deploy
```

Caso o deploy falhe
```
solana program close --buffers
```