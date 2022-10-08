Armazenando Valores em Variáveis
___________________________________

Próximo passo, vamos criar um local para armazenar a entrada do usuário:


let mut palpite = String::new();
Agora o programa está ficando interessante! Tem muita coisa acontecendo nesta pequena linha. Repare que esta é uma declaração let, que é usada para criar variáveis. Segue outro exemplo:


let foo = bar;
Essa linha cria uma nova variável chamada foo, e a vincula ao valor bar. Em Rust, variáveis são imutáveis por padrão. O exemplo a seguir mostra como usar mut antes do nome da variável para torná-la mutável:



let foo = 5; // imutável
let mut bar = 5; // mutável
Nota: A sintaxe // inicia um comentário, que continua até o fim da linha. Rust ignora tudo o que estiver nos comentários.
__________________________________________________

Agora você sabe que let mut palpite vai introduzir uma variável mutável de nome palpite. No outro lado do símbolo = está o valor ao qual palpite está vinculado, que é o resultado da chamada String::new, uma função que retorna uma nova instância de String. String é um tipo fornecido pela biblioteca padrão que representa uma cadeia expansível de caracteres codificados em UTF-8.

A sintaxe :: na linha ::new indica que new é uma função associada do tipo String. Uma função associada é implementada sobre um tipo, neste caso String, em vez de uma instância particular de String. Algumas linguagens dão a isso o nome método estático.

Esta função new() cria uma nova String vazia. Você encontrará uma função new() em muitos tipos, já que é um nome comum para uma função que produz um novo valor de algum tipo.

Para resumir, a linha let mut palpite = String::new(); criou uma variável mutável que está atualmente vinculada a uma nova instância vazia de uma String. Ufa!

Lembre-se de que incluímos a funcionalidade de entrada/saída da biblioteca padrão por meio do use std::io; na primeira linha do programa. Agora vamos chamar uma função associada, stdin, em io:


io::stdin().read_line(&mut palpite)
    .expect("Falha ao ler entrada");
Se não tivéssemos a linha use std::io no início do programa, poderíamos ter escrito esta chamada como std::io::stdin. A função stdin retorna uma instância de std::io::Stdin, um tipo que representa um manipulador (handle) da entrada padrão do seu terminal.

A próxima parte do código, .read_line(&mut palpite), chama o método read_line do handle da entrada padrão para obter entrada do usuário. Também estamos passando um argumento para read_line: &mut palpite.

O trabalho da função read_line é receber o que o usuário digita na entrada padrão e colocar isso numa string, por isso ela recebe essa string como argumento. A string do argumento deve ser mutável para que o método consiga alterar o seu conteúdo, adicionando a entrada do usuário.

O símbolo & indica que o argumento é uma referência, o que permite múltiplas partes do seu código acessar um certo dado sem precisar criar várias cópias dele na memória. Referências são uma característica complexa, e uma das maiores vantagens do Rust é o quão fácil e seguro é usar referências. Você não precisa conhecer muitos desses detalhes para finalizar esse programa. O Capítulo 4 vai explicar sobre referências de forma mais aprofundada. Por enquanto, tudo que você precisa saber é que, assim como as variáveis, referências são imutáveis por padrão. Por isso, precisamos escrever &mut palpite, em vez de apenas &palpite, para fazer com que o palpite seja mutável.

Ainda não finalizamos completamente esta linha de código. Embora esta seja uma única linha de texto, é apenas a primeira parte de uma linha lógica de código. A segunda parte é a chamada para este método:


.expect("Falha ao ler entrada");
Quando você chama um método com a sintaxe .foo(), geralmente é bom introduzir uma nova linha e outro espaço para ajudar a dividir linhas muito compridas. Poderíamos ter feito assim:


io::stdin().read_line(&mut palpite).expect("Falha ao ler entrada");
Porém, uma linha muito comprida fica difícil de ler. Então é melhor dividirmos a linha em duas, uma para cada método chamado. Agora vamos falar sobre o que essa linha faz.

Tratando Potenciais Falhas com o Tipo Result
Como mencionado anteriormente, read_line coloca o que o usuário escreve dentro da string que passamos como argumento, mas também retorna um valor - neste caso, um io::Result. Rust tem uma variedade de tipos com o nome Result em sua biblioteca padrão: um Result genérico e as versões específicas dos submódulos, como io::Result.

Os tipos Result são enumerações, comumente chamadas de enums. Uma enumeração é um tipo que pode ter um conjunto fixo de valores, os quais são chamados de variantes da enum. O Capítulo 6 vai abordar enums em mais detalhes.

Para Result, as variantes são Ok ou Err. Ok indica que a operação teve sucesso, e dentro da variante Ok está o valor resultante. Err significa que a operação falhou, e contém informações sobre como ou por que isso ocorreu.

O propósito destes tipos Result é codificar informações de manipulação de erros. Valores do tipo Result, assim como qualquer tipo, possuem métodos definidos. Uma instância de io::Result tem um método expect que você pode chamar. Se esta instância de io::Result é um Err, expect vai terminar o programa com erro e mostrar a mensagem que você passou como argumento ao expect. Se o método read_line retornar um Err, provavelmente seria o resultado de um erro vindo do sistema operacional que está por trás. Se esta instância de io::Result é um Ok, expect vai obter o valor contido no Ok e retorná-lo para que você possa usá-lo. Neste caso, o valor é o número de bytes dos dados que o usuário inseriu através da entrada padrão.

Se não chamarmos expect, nosso programa vai compilar, mas vamos ter um aviso:


$ cargo build
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut palpite);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
Rust avisa que não usamos o valor Result, retornado por read_line, indicando que o programa deixou de tratar um possível erro. A maneira correta de suprimir o aviso é realmente escrevendo um tratador de erro, mas como queremos que o programa seja encerrado caso ocorra um problema, podemos usar expect. Você aprenderá sobre recuperação de erros no Capítulo 9.

Exibindo Valores com Curingas do println!
Tirando a chave que delimita a função main, há apenas uma linha mais a ser discutida no código que fizemos até agora, que é a seguinte:


println!("Você disse: {}", guess);
Esta linha imprime a string na qual salvamos os dados inseridos pelo usuário. O {} é um curinga que reserva o lugar de um valor. Você pode imprimir mais de um valor usando {}: o primeiro conjunto de {} guarda o primeiro valor listado após a string de formatação, o segundo conjunto guarda o segundo valor, e assim por diante. Imprimir múltiplos valores em uma só chamada a println! seria assim:



let x = 5;
let y = 10;

println!("x = {} e y = {}", x, y);
Esse código imprime x = 5 e y = 10.

Testando a Primeira Parte
Vamos testar a primeira parte do jogo de advinhação. Você pode executá-lo usando cargo run:


$ cargo run
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/jogo_de_advinhacao`
Advinhe o número!
Digite o seu palpite.
6
Você disse: 6
Nesse ponto, a primeira parte do jogo está feita: podemos coletar entrada do teclado e mostrá-la na tela.

Gerando um Número Secreto
A seguir, precisamos gerar um número secreto que o usuário vai tentar advinhar. O número secreto deve ser diferente a cada execução, para que o jogo tenha graça em ser jogado mais de uma vez. Vamos usar um número aleatório entre 1 e 100, para que o jogo não seja tão difícil. Rust ainda não inclui uma funcionalidade de geração de números aleatórios em sua biblioteca padrão. Porém, a equipe Rust fornece um crate rand.

Usando um Crate para Ter Mais Funcionalidades
Lembre-se que um crate é um pacote de código Rust. O projeto que estamos construindo é um crate binário, que é um executável. Já o rand é um crate de biblioteca, que contém código cujo objetivo é ser usado por outros programas.

É no uso de crates externos que Cargo realmente brilha. Antes que possamos escrever o código usando rand, precisamos modificar o arquivo Cargo.toml para incluir o crate rand como uma dependência. Abra o arquivo e adicione esta linha no final, abaixo do cabeçalho da seção [dependencies] que o Cargo criou para você:

Arquivo: Cargo.toml


[dependencies]

rand = "0.3.14"
No arquivo Cargo.toml, tudo que vem depois de um cabeçalho é parte de uma seção que segue até o início de outra. A seção [dependencies] é onde você diz ao Cargo de quais crates externos o seu projeto depende, e quais versões desses crates você exige. Neste caso, especificamos o crate rand com a versão semântica 0.3.14. Cargo compreende Versionamento Semântico (às vezes chamado SemVer), um padrão para escrever números de versões. O número 0.3.14 é, na verdade, uma forma curta de escrever ^0.3.14, que significa "qualquer versão que tenha uma API pública compatível com a versão 0.3.14".

Agora, sem mudar código algum, vamos compilar nosso projeto, conforme mostrado na Listagem 2-2:


$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
Listagem 2-2: Resultado da execução de cargo build depois de adicionar o crate rand como dependência.

Talvez pra você apareçam versões diferentes (mas elas são todas compatíveis com o código, graças ao Versionamento Semântico!), e as linhas talvez apareçam em ordem diferente.

Agora que temos uma dependência externa, Cargo busca as versões mais recentes de tudo no registro, que é uma cópia dos dados do Crates.io. Crates.io é onde as pessoas do ecossistema Rust postam seus projetos open source para que os outros possam usar.

Após atualizar o registro, Cargo verifica a seção [dependencies] e baixa todas as que você não tem ainda. Neste caso, embora tenhamos listado apenas rand como dependência, o Cargo também puxou uma cópia da libc, porque rand depende da libc para funcionar. Depois de baixá-las, o Cargo as compila e então compila nosso projeto.

Se, logo em seguida, você executar cargo build novamente sem fazer mudanças, não vai aparecer nenhuma mensagem de saída. O Cargo sabe que já baixou e compilou as dependências, e você não alterou mais nada sobre elas no seu arquivo Cargo.toml. Cargo também sabe que você não mudou mais nada no seu código, e por isso não o recompila. Sem nada a fazer, ele simplesmente sai. Se você abrir src/main.rs, fizer uma modificação trivial, salvar e compilar de novo, vai aparecer uma mensagem de apenas duas linhas:


$ cargo build
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
Essas linhas mostram que o Cargo só atualiza o build com a sua pequena mudança no arquivo src/main.rs. Suas dependências não mudaram, então o Cargo sabe que pode reutilizar o que já tiver sido baixado e compilado para elas. Ele apenas recompila a sua parte do código.

O Arquivo Cargo.lock Garante Builds Reproduzíveis
O Cargo tem um mecanismo que assegura que você pode reconstruir o mesmo artefato toda vez que você ou outra pessoa compilar o seu código. O Cargo vai usar apenas as versões das dependências que você especificou, até que você indique o contrário. Por exemplo, o que acontece se, na semana que vem, sair a versão v0.3.15 contendo uma correção de bug, mas também uma regressão que não funciona com o seu código?

A resposta para isso está no arquivo Cargo.lock, que foi criado na primeira vez que você executou cargo build, e agora está no seu diretório jogo_de_advinhacao. Quando você compila o seu projeto pela primeira vez, o Cargo descobre as versões de todas as dependências que preenchem os critérios e então as escreve no arquivo Cargo.lock. Quando você compilar o seu projeto futuramente, o Cargo verá que o arquivo Cargo.lock existe e usará as versões especificadas lá, em vez de refazer todo o trabalho descobrir as versões novamente. Isto lhe permite ter um build reproduzível automaticamente. Em outras palavras, seu projeto vai continuar com a versão 0.3.14 até que você faça uma atualização explícita, graças ao arquivo Cargo.lock.

Atualizando um Crate para Obter uma Nova Versão
Quando você quiser atualizar um crate, o Cargo tem outro comando, update, que faz o seguinte:

Ignora o arquivo Cargo.lock e descobre todas as versões mais recentes que atendem as suas especificações no Cargo.toml.
Se funcionar, o Cargo escreve essas versões no arquivo Cargo.lock.
Mas, por padrão, o Cargo vai procurar as versões maiores que 0.3.0 e menores que 0.4.0. Se o crate rand já tiver lançado duas novas versões, 0.3.15 e 0.4.0, você verá a seguinte mensagem ao executar cargo update:


$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
Nesse ponto, você vai notar também uma mudança no seu arquivo Cargo.lock dizendo que a versão do crate rand que você está usando agora é a 0.3.15.

Se você quisesse usar a versão 0.4.0, ou qualquer versão da série 0.4.x do rand, você teria que atualizar o seu Cargo.toml dessa forma:


[dependencies]

rand = "0.4.0"
Na próxima vez que você executar cargo build, o Cargo vai atualizar o registro de crates disponíveis e reavaliar os seus requisitos sobre o rand de acordo com a nova versão que você especificou.

Há muito mais a ser dito sobre Cargo e o seu ecossistema que vai ser discutido no Capítulo 14, mas por ora isto é tudo que você precisa saber. Cargo facilita muito reutilizar bibliotecas, de forma que os rustáceos consigam escrever projetos menores que são montados a partir de diversos pacotes.

Gerando um Número Aleatório
Agora vamos usar, de fato, o rand. O próximo passo é atualizar o src/main.rs conforme mostrado na Listagem 2-3:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);
}
Listagem 2-3: Mudanças necessárias do código para gerar um número aleatório.

Estamos adicionando a linha extern crate rand ao topo do arquivo para indicar ao Rust que estamos usando uma dependência externa. Isto também é equivalente a um use rand;, assim podemos chamar qualquer coisa que esteja no crate rand prefixando-a com rand::.

Em seguida, adicionamos outra linha use: use rand::Rng. Rng é um trait que define métodos a serem implementados pelos geradores de números aleatórios, e esse trait deve estar dentro do escopo para que possamos usar esses métodos. O Capítulo 10 vai abordar traits em mais detalhes.

Tem outras duas linhas que adicionamos no meio. A função rand::thread_rng nos dá o gerador de números aleatórios que vamos usar, um que é local à thread corrente e que é inicializado pelo sistema operacional. Depois, vamos chamar o método gen_range no gerador de números aleatórios. Esse método está definido pelo trait Rng que trouxemos ao escopo por meio do use rand::Rng. Este método recebe dois argumentos e gera um número aleatório entre eles. Ele inclui o limite inferior mas exclui o superior, então precisamos passar 1 e 101 para obter um número de 1 a 100.

Saber quais traits devem ser usadas e quais funções e métodos de um crate devem ser chamados não é nada trivial. As instruções de como usar um crate estão na documentação de cada um. Outra coisa boa do Cargo é que você pode rodar o comando cargo doc --open que vai construir localmente a documentação fornecida por todas as suas dependências e abrí-las no seu navegador. Se você estiver interessado em outras funcionalidades do crate rand, por exemplo, execute cargo doc --open e clique em rand, no menu ao lado esquerdo.

A segunda linha que adicionamos imprime o número secreto. Isto é útil enquanto estamos desenvolvendo o programa para podermos testá-lo, mas vamos retirá-la da versão final. Um jogo não é muito interessante se ele mostra a resposta logo no início!

Tente rodar o programa algumas vezes:


$ cargo run
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/jogo_de_advinhacao`
Advinhe o número!
O número secreto é: 7
Digite o seu palpite.
4
Você disse: 4
$ cargo run
     Running `target/debug/jogo_de_advinhacao`
Advinhe o número!
O número secreto é: 83
Digite o seu palpite.
5
Você disse: 5
Você já deve obter números aleatórios diferentes, e eles devem ser todos entre 1 e 100. Bom trabalho!

Comparando o Palpite com o Número Secreto
Agora que nós temos a entrada do usuário e o número secreto, vamos compará-los. Esta estapa é mostrada na Listagem 2-4:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito baixo!"),
        Ordering::Greater => println!("Muito alto!"),
        Ordering::Equal => println!("Você acertou!"),
    }
}
Listagem 2-4: Tratando os possíveis resultados da comparação de dois números.

A primeira novidade aqui é outro use, que traz ao escopo um tipo da biblioteca padrão chamado std::cmp::Ordering. Ordering é outra enum, igual a Result, mas as suas variantes são Less, Greater e Equal (elas significam menor, maior e igual, respectivamente). Estes são os três possíveis resultados quando você compara dois valores.

Depois, adicionamos cinco novas linhas no final que usam o tipo Ordering:


match palpite.cmp(&numero_secreto) {
    Ordering::Less => println!("Muito baixo!"),
    Ordering::Greater => println!("Muito alto!"),
    Ordering::Equal => println!("Você acertou!"),
}
O método cmp compara dois valores, e pode ser chamado a partir de qualquer coisa que possa ser comparada. Ele recebe uma referência de qualquer coisa que você queira comparar. Neste caso, está comparando o palpite com o numero_secreto. cmp retorna uma variante do tipo Ordering, que trouxemos ao escopo com use. Nós usamos uma expressão match para decidir o que fazer em seguida, com base em qual variante de Ordering foi retornada pelo método cmp, que foi chamado com os valores palpite e numero_secreto.

Uma expressão match é composta de braços. Um braço consiste em um padrão mais o código que deve ser executado se o valor colocado no início do match se encaixar no padrão deste braço. O Rust pega o valor passado ao match e o compara com o padrão de cada braço na sequência. A expressão match e os padrões são ferramentas poderosas do Rust que lhe permitem expressar uma variedade de situações que seu código pode encontrar, e ajuda a assegurar que você tenha tratado todas elas. Essas ferramentas serão abordadas em detalhes nos capítulos 6 e 18, respectivamente.

Vamos acompanhar um exemplo do que aconteceria na expressão match usada aqui. Digamos que o usuário tenha colocado 50 como palpite, e o número secreto aleatório desta vez é 38. Quando o código compara 50 com 38, o método cmp vai retornar Ordering::Greater, porque 50 é maior que 38. Ordering::Greater é o valor passado ao match. Ele olha para o padrão Ordering::Less do primeiro braço, mas o valor Ordering::Greater não casa com Ordering::Less, então ele ignora o código desse braço e avança para o próximo. Já o padrão do próximo braço, Ordering::Greater, casa com Ordering::Greater! O código associado a este braço vai ser executado e mostrar Muito alto! na tela. A expressão match termina porque já não tem mais necessidade de verificar o último braço nesse caso particular.

Porém, o código da Listagem 2-4 ainda não vai compilar. Vamos tentar:


$ cargo build
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match palpite.cmp(&numero_secreto) {
   |                       ^^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `jogo_de_advinhacao`.
O que este erro está dizendo é que temos tipos incompatíveis. Rust tem um sistema de tipos forte e estático. Porém, Rust também tem inferência de tipos. Quando escrevemos let palpite = String::new(), Rust foi capaz de inferir que palpite deveria ser uma String, então ele não nos faz escrever o tipo. O numero_secreto, por outro lado, é de um tipo numérico. Existem alguns tipos numéricos capazes de guardar um valor entre 1 e 100: i32, que é um número de 32 bits; u32, um número de 32 bits sem sinal; i64, um número de 64 bits; e mais alguns outros. O tipo numérico padrão do Rust é i32, que é o tipo do numero_secreto, a não ser que adicionemos, em algum lugar, uma informação de tipo que faça o Rust inferir outro tipo numérico. A razão do erro é que o Rust não pode comparar uma string e um tipo numérico.

Em última análise, queremos converter a String que lemos como entrada em um tipo numérico de verdade, de forma que possamos compará-lo numericamente com o palpite. Podemos fazer isso com mais duas linhas no corpo da função main:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    let palpite: u32 = palpite.trim().parse()
        .expect("Por favor, digite um número!");

    println!("Você disse: {}", palpite);

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito baixo!"),
        Ordering::Greater => println!("Muito alto!"),
        Ordering::Equal => println!("Você acertou!"),
    }
}
As duas linhas novas são:


let palpite: u32 = palpite.trim().parse()
    .expect("Por favor, digite um número!");
Nós criamos uma variável chamada palpite. Mas espera, o programa já não tinha uma variável chamada palpite? Sim, mas o Rust nos permite sombrear o palpite anterior com um novo. Isto é geralmente usado em situações em que você quer converter um valor de um tipo em outro. O sombreamento nos permite reutilizar o nome palpite, em vez de nos forçar a criar dois nomes únicos como palpite_str e palpite, por exemplo. (O Capítulo 3 vai cobrir sombreamento em mais detalhes).

Nós vinculamos palpite à expressão palpite.trim().parse(). O palpite, na expressão, refere-se ao palpite original contendo a String de entrada do usuário. O método trim, em uma instância de String, vai eliminar quaisquer espaços em branco no início e no fim. u32 pode conter apenas caracteres numéricos, mas o usuário precisa pressionar Enter para satisfazer o read_line. Quando o usuário pressiona Enter, um caractere de nova linha é inserido na string. Por exemplo, se o usuário digitar 5 e depois Enter, palpite ficaria assim: 5\n. O \n representa uma linha nova, a tecla Enter. O método trim elimina o \n, deixando apenas 5.

O método parse em strings converte uma string para algum tipo de número. Dado que ele pode interpretar uma variedade de tipos numéricos, precisamos dizer ao Rust qual o tipo exato de número nós queremos, e para isso usamos let palpite: u32. Os dois pontos (:) depois de palpite informam ao Rust que estamos anotando seu tipo. O Rust tem alguns tipos numéricos embutidos, o u32 visto aqui é um inteiro de 32 bits sem sinal. É uma boa escolha padrão para um número positivo pequeno. Você vai aprender sobre outros tipos numéricos no Capítulo 3. Além disso, a anotação u32 neste programa de exemplo e a comparação com numero_secreto significam que o Rust vai inferir que numero_secreto também deve ser um u32. Então agora a comparação vai ser feita entre valores do mesmo tipo!

A chamada para parse poderia facilmente causar um erro. Por exemplo, se a string contiver A👍%, não haveria como converter isto em um número. Como ele pode falhar, o método parse retorna um Result, assim como o método read_line, conforme discutido anteriormente na seção "Tratando Potenciais Falhas com o Tipo Result. Vamos tratar este Result da mesma forma usando o método expect de novo. Se o parse retornar uma variante Err da enum Result, por não conseguir criar um número a partir da string, a chamada ao expect vai causar um crash no jogo e exibir a mensagem que passamos a ele. Se o parse conseguir converter uma string em um número, ele vai retornar a variante Ok da enum Result e expect vai retornar o número que queremos extrair do valor Ok.

Agora vamos executar o programa!


$ cargo run
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/jogo_de_advinhacao`
Advinhe o número!
O número secreto é: 58
Digite o seu palpite.
  76
Você disse: 76
Muito alto!
Boa! Até mesmo colocando alguns espaços antes de digitar o palpite, o programa ainda descobriu que o palpite do usuário é 76. Execute o programa mais algumas vezes para verificar os diferentes comportamentos com diferentes tipos de entrada: advinhe o número corretamente, digite um número muito alto, e digite um número muito baixo.

Agora já temos a maior parte do jogo funcionando, mas o usuário só consegue dar um palpite uma vez. Vamos mudar isso adicionando laços!

Permitindo Múltiplos Palpites Usando Looping
A palavra-chave loop nos dá um laço (loop) infinito. Use-a para dar aos usuários mais chances de advinhar o número:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = palpite.trim().parse()
            .expect("Por favor, digite um número!");

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => println!("Você acertou!"),
        }
    }
}
Como você pode ver, movemos tudo para dentro do laço a partir da mensagem pedindo o palpite do usuário. Certifique-se de indentar essas linhas mais quatro espaços cada uma, e execute o programa novamente. Repare que há um novo problema, porque o programa está fazendo exatamente o que dissemos para ele fazer: pedir sempre outro palpite! Parece que o usuário não consegue sair!

O usuário pode sempre interromper o programa usando as teclas ctrl-c. Mas há uma outra forma de escapar deste monstro insaciável que mencionamos na discussão do método parse, na seção "Comparando o Palpite com o Número Secreto": se o usuário fornece uma resposta não-numérica, o programa vai sofrer um crash. O usuário pode levar vantagem disso para conseguir sair, como mostrado abaixo:


$ cargo run
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
     Running `target/jogo_de_advinhacao`
Advinhe o número!
O número secreto é: 59
Digite o seu palpite.
45
Você disse: 45
Muito baixo!
Digite o seu palpite.
60
Você disse: 60
Muito alto!
Digite o seu palpite.
59
Você disse: 59
Você acertou!
Digite o seu palpite.
sair
thread 'main' panicked at 'Por favor, digite um número!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/jogo_de_advinhacao` (exit code: 101)
Digitar sair, na verdade, sai do jogo, mas isso também acontece com qualquer outra entrada não numérica. Porém, isto não é o ideal. Queremos que o jogo termine automaticamente quando o número é advinhado corretamente.

Saindo Após um Palpite Correto
Vamos programar o jogo para sair quando o usuário vencer, colocando um break:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = palpite.trim().parse()
            .expect("Por favor, digite um número!");

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
Adicionando a linha break após o Você acertou!, o programa vai sair do laço quando o usuário advinhar corretamente o número secreto. Sair do laço também significa sair do programa, pois o laço é a última parte da main.

Tratando Entradas Inválidas
Para refinar ainda mais o comportamento do jogo, em vez de causar um crash no programa quando o usuário insere uma entrada não numérica, vamos fazer o jogo ignorá-la para que o usuário possa continuar tentando. Podemos fazer isso alterando a linha em que o palpite é convertido de String para u32:


let palpite: u32 = match palpite.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
Trocando uma chamada a expect por uma expressão match é a forma como você geralmente deixa de causar um crash em um erro e passa a tratá-lo, de fato. Lembre-se que o método parse retorna um valor do tipo Result, uma enum que contém a variante Ok ou Err. Estamos usando um match aqui, assim como fizemos com o Ordering resultante do método cmp.

Se o parse consegue converter a string em um número, ele vai retornar um Ok contendo o número resultante. Esse valor Ok vai casar com o padrão do primeiro braço, e o match vai apenas retornar o valor num produzido pelo parse e colocado dentro do Ok. Esse número vai acabar ficando exatamente onde queremos, na variável palpite que estamos criando.

Se o parse não conseguir converter a string em um número, ele vai retornar um Err que contém mais informações sobre o erro. O valor Err não casa com o padrão Ok(num) do primeiro braço do match, mas casa com o padrão Err(_) do segundo braço. O _ é um valor "pega tudo". Neste exemplo, estamos dizendo que queremos casar todos os valores Err, não importa qual informação há dentro deles. Então o programa vai executar o código do segundo braço, continue, que significa ir para a próxima iteração do loop e pedir outro palpite. Efetivamente, o programa ignora todos os erros que o parse vier a encontrar!

Agora, tudo no programa deve funcionar como esperado. Vamos tentar executá-lo usando o comando cargo run:


$ cargo run
   Compiling jogo_de_advinhacao v0.1.0 (file:///projects/jogo_de_advinhacao)
     Running `target/jogo_de_advinhacao`
Advinhe o número!
O número secreto é: 61
Digite o seu palpite.
10
Você disse: 10
Muito baixo!
Digite o seu palpite.
99
Você disse: 99
Muito alto!
Digite o seu palpite.
foo
Digite o seu palpite.
61
Você disse: 61
Você acertou!
Demais! Com apenas um último ajuste, vamos finalizar o jogo de adivinhação: lembre-se que o programa ainda está mostrando o número secreto. Isto foi bom para testar, mas estraga o jogo. Vamos apagar o println! que revela o número secreto. A Listagem 2-5 mostra o código final:

Arquivo: src/main.rs


extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
Listagem 2-5: Código completo do jogo de advinhação.

Resumo
Neste ponto, você construiu com sucesso o jogo de adivinhação! Parabéns!

Este projeto foi uma forma prática de apresentar vários conceitos novos de Rust: let, match, métodos, funções associadas, uso de crates externos, e outros. Nos próximos capítulos, você vai aprender sobre esses conceitos em mais detalhes. O Capítulo 3 aborda conceitos que a maioria das linguagens de programação tem, como variáveis, tipos de dados e funções, e mostra como usá-los em Rust. O Capítulo 4 explora posse (ownership), que é a característica do Rust mais diferente das outras linguagens. O Capítulo 5 discute structs e a sintaxe de métodos, e o Capítulo 6 se dedica a explicar enums.



