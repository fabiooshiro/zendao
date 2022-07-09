# John Foot Front

As instruções serão assinadas e enviadas para uma função lambda off-chain para que não seja necessário pagar taxas na web3 para operações não críticas.

Ou então a gente assina uma espécie de jwt somente 1x e confere no backend.

## React Functional Component
https://fettblog.eu/typescript-react-why-i-dont-use-react-fc/

## Mac M1

Rode: 
`npm add @parcel/css-darwin-arm64`
para incluir o 
"@parcel/css-darwin-arm64": "^1.8.1"
no package.json

## Deploy

Basta jogar na master que o pipelines cuida de atualizar em:  
https://califooters.calindra.com.br/

### Deploy under the hood

Dentro do arquivo `bitbucket-pipelines.yml` contém instruções para transpilar/buildar o front e subir no s3 johnfoot na pasta front

Foi configurado um CloudFront `E3A5DQHZAVYZHZ` manualmente:

* Alternate domain names: califooters.calindra.com.br
* Custom SSL certificate: *.calindra.com.br 
* Origin path: /front

## Front

Campo de futebol em CSS 3D
https://codepen.io/fabiooshiro/pen/gOovPME

Campo de futebol 2D
https://codepen.io/fabiooshiro/pen/rNpJxzw

baixado em ./research
