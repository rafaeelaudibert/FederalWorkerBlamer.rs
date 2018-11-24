import csv

with open("csv/201809_Remuneracao.csv", "r") as f:
    dados = csv.reader(f, delimiter=";")
    lista = list(dados)[1:]
    ordenada = sorted(lista, key= lambda val : (val[4], int(val[2])))

    with open("csv/201809_RemuneracaoParsed.csv", "w") as write_file:
        writer = csv.writer(write_file, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        for element in ordenada:
            writer.writerow(element)
