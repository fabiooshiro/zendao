# Paradigmas de Programação
TL:DR; [Programming Across Paradigms • Anjana Vakil • GOTO 2017](https://www.youtube.com/watch?v=Pg3UeB-5FdA)

## Assembler
Muito baixo nível, fora do escopo inicial do curso.

## Programação estruturada (while if else)
Alto nível, muito melhor do que assembler. GOTO é/era terrível de se seguir e nesse paradigma as estruturas ficam mais fáceis de se entender do que na macarronada do assembler.

## P.O.O.

Os programas ou suas partes ainda assim eram difíceis de serem reutilizados e evoluídos no paradigma estruturado. As variáveis eram muitas vezes `globais`/`públicas` e ficava complexo debugar e entender qual parte do código alterou alguma variável que começou a quebrar o sistema.

Abstração, herança  
Encapsulamento  
Polimorfismo  

### Generics &lt;T&gt;

A P.O.O. ajudou bastante na reutilização de código, como por exemplo, nas collections do Java, vc não precisa mais implementar ArrayList, HashMap, TreeMap, etc. Porém ainda era chato ter que ficar dando cast ou especializando um Array ou Map para um tipo específico.

```Java
ArrayList array = new ArrayList();
array.add("primeiro valor");
String firstValue = (String) array.get(0);
```
O (String) é a operação de cast, velhos tempos.

```Java
class StringArray extends ArrayList {
    public void add(String item) {
        super.add(item);
    }
    
    public String get(int i) {
        return (String) super.get(i);
    }
}
```

```Java
StringArray array = new StringArray();
array.add("primeiro valor");
String firstValue = array.get(0);
```

Imagina o corno-job de ter que criar uma classe CarMap, BikeMap, BallMap, etc.  
Então tiveram a brilhante idéia de inventar os generics, que nada mais é do que colocar o tipo dinamicamente. Você encontra isso em várias outras linguagens como TypeScript.

```ts
let dictionary = new Map<string, string>();
dictionary.set("hi", "oi");
```

É ótimo poder ter um &lt;TipoDinamico&gt;

Na vida real muita gente passou a fazer DAO, no sentido de [Data Access Object](https://en.wikipedia.org/wiki/Data_access_object) de maneira genérica para fazer as operações de persistência, vulgarmente chamado de CRUD.

```ts
class GenericDAO<T> {
    async create(obj: T) {
        await collection.insertOne(obj)
    }
    async update(obj: T) {
        await collection.updateOne(obj._id, obj)
    }
    async read(id: ObjectID) {
        await collection.findOne(id)
    }
    async delete(obj: T) {
        await collection.deleteOne(obj._id)
    }
}
```

## A.O.P. (arroba oriented)
Mesmo com toda essa evolução, sempre a gente pode melhorar as coisas :-)  
E por falar nisso, ainda havia um problema sem uma solução elegante.  Imagina operações no banco de dados que vc precisa abrir uma transação e se algo der errado vc dá rollback ou se tudo der certo vc faz commit.
```ts
async exchange(id1: any, id2: any, val: number) {
  let session = this.startTransactionWithCollection('coins')
  try {
    session.updateOne(id1, { $inc: { val+ +val }})
    session.updateOne(id2, { $inc: { val: -val }})
    session.commitTransaction()
  } catch(e) {
    session.abortTransaction() //rollback
  } finally {
    session.endSession()
  }
}
// * código meramente ilustrativo
```
Imagina se a gente pudesse trocar o código acima por:
```ts
@Transactional()
async exchange(id1: number, id2: number, val: number) {
  let collection = await this.getCollection('coins')
  collection.updateOne(id1, { $inc: { val+ +val }})
  collection.updateOne(id2, { $inc: { val: -val }})
}
// * código meramente ilustrativo
```

Nas operações comuns entre vários objetos, com os mesmos métodos, até daria para encapsular em uma classe abstrata e usar generics, como no exemplo do CRUD, mas sempre vai existir um método, ou melhor, vários métodos que precisam "herdar" um aspecto transacional que não dá para solucionar com herança.

Arroba é só pra transação? Claro que não.

## Funcional () =>

A orientação a objetos ficou complexa, e se a gente usasse alguns conceitos da matemática e de quebra fizesse algo mais simples?  
Lembro do Andre Kelmanson falando dos [POJOs](https://pt.wikipedia.org/wiki/Plain_Old_Java_Objects) que na real são as velhas estruturas, sem os métodos, que transitam entre as camadas do sistema. Essas camadas de sistema são normalmente stateless, ou seja, não possuem os atributos, só a lógica.  
Concordo plenamente, nas arquiteturas desse atual ano de 2022, vemos que a lógica foi separada dos atributos. O que segundo prega a POO deveríamos ter um objeto com atributos e métodos juntos.

```ts
let car = new Car()
await car.moveTo(destination)
await car.save()
```
Mas muitas vezes na prática é melhor implementar separando as responsabilidades

```ts
let car = new Car()
carService.move(car, destination)
carRepository.save(car)
```
Com essa divisão, se no futuro vc precisar trocar o banco de dados (nesses últimos anos fiz isso pra caramba) é só alterar o repository e pronto. Obviamente também existe ganho no dia a dia, pois fica mais fácil encontrar o trecho de código que vc precisa. É problema na hora de atualizar? Deve estar no repository. É erro de lógica de negócio? Deve estar no serviço.

O pessoal do React está aplicando fortemente essa idéia (ainda escrita com acento pois eu sou um velho rebelde) e na maioria dos casos faz muito sentido.

Veja esse exemplo em POO:

```js
class Wellcome extends React.Component {
  render() {
    return <h1>Olá, {this.props.name}</h1>;
  }
}
```
E veja isso no estilo funcional:
```js
function Welcome({ name }) {
  return <h1>Olá, {name}</h1>;
}
```
Essa segunda implementação traz um outro conceito interessante: `pure functions`, que nada mais é do que se pensar nas funções matemáticas, se a entrada é a mesma o resultado é sempre o mesmo.

E para saber quem mexeu nas minhas variáveis a solução tb é simples, mas um pensamento totalmente fora da caixa: e se a gente simplesmente não alterasse nada? Kkkkk  
Bem-vindo a era da `imutabilidade`, em vez de alterar a variável/objeto do amiguinho, faz uma cópia pra vc! E agora que a variável é só sua vc altera como quiser.

