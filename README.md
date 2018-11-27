# TrabalhoFinalCPD
Esse é o repositório para o Trabalho Final da cadeira de Classificação e Pesquisa de Dados do curso de Ciência da Computação - UFRGS

## Setup
Para utilizar essa aplicação você precisará baixar e instalar o compilador para a linguagem Rust e a ferramenta Cargo [daqui](https://www.rust-lang.org/pt-BR/install.html). OBS: O Cargo já vem junto com o compilador Rust.
Também é necessário possuir o Python3 instalado

Você precisará baixar os dados dos funcionários públicos federais do site da [Transparência Federal](http://www.portaltransparencia.gov.br/download-de-dados/servidores). Você pode baixar qualquer um dos dados presentes ali, porém recomendo baixar o do mês de [Julho](http://www.portaltransparencia.gov.br/download-de-dados/servidores/201807_Servidores), já que o script em Python está pré-configurado para ele

Esses arquivos devem ser colocados dentro da pasta CSV.

## Como Usar
Primeiro, é necessário rodar o script em Python para configurar o arquivo de Remuneração de maneira que o programa em Rust consiga lê-lo. Altere o código em Python para o nome do arquivo que você baixar (por padrão temos o mes de Junho de 2018). Após, na raiz do projeto, pela linha de comando rode: `python lib/script.py`. Ele irá gerar o arquivo `csv/<ano_mes_dia>_RemuneracaoParsed.csv` que deverá ser usado na criação do banco de dados.

Após, para executar o programa você poderá rodar `cargo run --release -- -h` para poder ver o menu de ajuda com as possíveis utilizações do programa. Para começar, rode `cargo run --release -- -c csv/<ano><mes>_RemuneracaoParsed.csv csv/<ano><mes>_Cadastro.csv` para criar o banco de dados e as tries iniciais. Caso ocorra algum erro relacionado com "UTF8", abra o arquivo csv referente ao cadastro, e no primeiro nome, altere o PRIMEIRO texto que contem "Sem informa��o" para "Sem informação" (porque ele não consegue entender esse, mas consegue entender todos os próximos é um belo mistério).

Rodando `cargon run ...` você está compilando o programa toda vez. Por padrão, já foi deixado um arquivo compilado (para Windows) na root do projeto, dessa maneira você poderá utilizá-lo a partir da linha de comando, utilizando as mesmas flags que podem ser utilizadas com o programa.
