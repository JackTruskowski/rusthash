#small script to average throughput over 5 runs

import csv

sizes = ["8bit", "16bit", "32bit", "64bit"]

for size in sizes:

    inserts = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    finds = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

    for i in range(1, 6):
        print("Averaging 5 runs for " + size)
        with open(size + "-" + str(i) + ".csv", mode="r") as csv_in:
            csv_reader = csv.reader(csv_in)
            line_count = 0
            for row in csv_reader:
                for i, token in enumerate(row):
                    if line_count == 0:
                        inserts[i] += float(token)
                    else:
                        finds[i] += float(token)
                line_count += 1

    for idx in range(len(inserts)):
        inserts[idx] = inserts[idx] / 5.0
        finds[idx] = finds[idx] / 5.0

    with open(size + "-final.csv", mode='wb') as csv_out:
        spamwriter = csv.writer(csv_out, delimiter=',')
        spamwriter.writerow(inserts)
        spamwriter.writerow(finds)
