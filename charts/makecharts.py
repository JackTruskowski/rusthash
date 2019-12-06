import csv
import matplotlib.pyplot as plt
import numpy as np

xs = ["1", "2", "4", "8"]
insert_ys = []
find_ys = []

with open("../hello-rust/out.csv", mode="r") as csv_file:
    print("hello world")
    csv_reader = csv.reader(csv_file)
    line_count = 0
    for row in csv_reader:
        for token in row:
            if line_count == 0:
                insert_ys.append(float(token))
            else:
                find_ys.append(float(token))
        line_count += 1


def insert():
    plt.figure(num = 3, figsize=(8, 5))
    plt.plot(xs, insert_ys, color='black', marker="s", linewidth=1.0)
    plt.xlabel("Threads")
    plt.ylabel("Speedup")
    plt.title("Insertion Performance")
    plt.show()

def find():
    plt.figure(num = 3, figsize=(8, 5))
    plt.plot(xs, find_ys, color='black', marker="s", linewidth=1.0)
    plt.xlabel("Threads")
    plt.ylabel("Speedup")
    plt.title("Find Performance")
    plt.show()

insert()
find()
