import csv
import matplotlib.pyplot as plt
import numpy as np

xs = [1, 2, 4, 8, 12, 16, 32, 48]
insert_ys = []
find_ys = []

filename = "out.csv"

with open("../hello-rust/" + filename, mode="r") as csv_file:
    csv_reader = csv.reader(csv_file)
    line_count = 0
    for row in csv_reader:
        for token in row:
            if line_count == 0:
                insert_ys.append(float(token))
            else:
                find_ys.append(float(token))
        line_count += 1


def insert(do_speedup):

    plt.figure(figsize=(8, 5))

    ys = []
    if do_speedup:
        plt.ylabel("Absolute Speedup\n(vs. built-in Rust HashMap)")
        for y_idx in range(1, len(insert_ys)):
            ys.append(insert_ys[y_idx] / insert_ys[0])
    else:
        plt.ylabel("Throughput (MOps/sec)")
        ys = insert_ys[1:]

    plt.plot(xs, ys, color='black', marker="s", linewidth=1.0)
    plt.xlabel("Threads")
    plt.title("Insertion Performance")
    plt.legend(['Folklore Rust'])
    plt.xticks(xs, xs)
    plt.grid(axis='x', linestyle='--')
    if do_speedup:
        plt.savefig("insert_speed.png")
    else:
        plt.savefig("insert_thru.png")

def find(do_speedup):

    plt.figure(figsize=(8, 5))

    ys = []
    if do_speedup:
        plt.ylabel("Absolute Speedup\n(vs. built-in Rust HashMap)")
        for y_idx in range(1, len(find_ys)):
            ys.append(find_ys[y_idx] / find_ys[0])
        print(ys)
    else:
        plt.ylabel("Throughput (MOps/sec)")
        ys = find_ys[1:]

    plt.plot(xs, ys, color='black', marker="s", linewidth=1.0)
    plt.xlabel("Threads")
    plt.title("Find Performance")
    plt.legend(['Folklore Rust'])
    plt.xticks(xs, xs)
    plt.grid(axis='x', linestyle='--')
    if do_speedup:
        plt.savefig(filename + "_find_speed.png")
    else:
        plt.savefig(filename + "_find_thru.png")

insert(True)
find(True)
insert(False)
find(False)
