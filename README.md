# Federal Worker Blamer
This repository contains the Final assignment for Data Search and Classification (INF01124) subject, done at [INF](https://inf.ufrgs.br)-[UFRGS](https://ufrgs.br).

## Setup
To use this application you will need to download and install Rust language compiler and the Cargo tool [from here](https://www.rust-lang.org/en/install.html). Note: Cargo already comes with the Rust compiler. You must also have Python 3.x installed, which already comes installed in Windows and most Linux distributions.

You will need to download brazilian federal government employees data from the [Federal Transparency](http://www.portaltransparencia.gov.br/download-de-dados/servidores) website . You can download any of the data present there, however I recommend downloading [July, 2018](http://www.portaltransparencia.gov.br/download-de-dados/servidores/201807_Servidores) data, since the script in Python is preconfigured for it.

These files must be placed inside the CSV folder.

## How to use it
First, you need to run the script in Python to configure the Remuneration file so that the program in Rust can read it. Change the code in Python to the name of the file you download (by default we have the month of July 2018). Then, at the root of the project, run the command line `python3 lib/script.py`. It will generate the `csv/<year_months_day>_Parsed.csv` file that should be used when creating the database.

Afterwards, to run the program you can run `cargo run --release -- -h` to see the help menu with possible uses of the program. To get started, run `cargo run -release -- -c csv/<ano><mes> _RemuneracaoParsed.csv csv/<ano><mes>_Cadastro.csv` to create the database and the initial Tries. If there is an error related to "UTF8", open the csv file with "Cadastro"in his name, and in the first name, change the FIRST text that contains "Sem informa��o" to "Sem informação" (why it can not understand this, but can understand all the next is a beautiful mystery).

Running `cargo run ...` you are compiling the program every time. By default, a compiled Windows file has already been left at the root of the project, so you can use it from the command line, using the same flags that can be used with the program.
