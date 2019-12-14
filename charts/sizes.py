import csv
import matplotlib.pyplot as plt
import numpy as np

xs = [1, 2, 4, 8, 12, 16, 32, 48]
charts = ["32bit", "64bit"]
markers = ["s", "^", "."]

data = {}
for size in charts:
    data[size] = ([], [])

for size in charts:
    with open("./data/" + size + ".csv", mode="r") as csv_file:
        csv_reader = csv.reader(csv_file)
        line_count = 0
        for row in csv_reader:
            for token in row:
                if line_count == 0:
                    data[size][0].append(float(token))
                else:
                    data[size][1].append(float(token))
            line_count += 1
    data[size] = (data[size][0][1:], data[size][1][1:])


def insert():


    plt.figure(figsize=(8, 5))

    # ys = []
    # if do_speedup:
    #     plt.ylabel("Absolute Speedup\n(vs. built-in Rust HashMap)")
    #     for y_idx in range(1, len(insert_ys)):
    #         ys.append(insert_ys[y_idx] / insert_ys[0])
    # else:
    #     plt.ylabel("Throughput (MOps/sec)")
    #     ys = insert_ys[1:]


    for idx, size in enumerate(charts):
        plt.plot(xs, data[size][0], color='black', marker=markers[idx], linewidth=1.0)
    plt.xlabel("Threads")
    plt.ylabel("Throughput (MOps/sec)")
    plt.title("Insertion Performance")
    plt.legend(charts)
    plt.xticks(xs, xs)
    plt.grid(axis='x', linestyle='--')
    plt.savefig("figs/size_insert_thru.png")

def find():

    plt.figure(figsize=(8, 5))

    # ys = []
    # if do_speedup:
    #     plt.ylabel("Absolute Speedup\n(vs. built-in Rust HashMap)")
    #     for y_idx in range(1, len(find_ys)):
    #         ys.append(find_ys[y_idx] / find_ys[0])
    #     print(ys)
    # else:
    #     plt.ylabel("Throughput (MOps/sec)")
    #     ys = find_ys[1:]
    for idx, size in enumerate(charts):
        plt.plot(xs, data[size][1], color='black', marker=markers[idx], linewidth=1.0)
    plt.xlabel("Threads")
    plt.ylabel("Throughput (MOps/sec)")
    plt.title("Find Performance")
    plt.legend(charts)
    plt.xticks(xs, xs)
    plt.grid(axis='x', linestyle='--')
    plt.savefig("figs/size_find_thru.png")

insert()
find()
