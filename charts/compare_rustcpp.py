import matplotlib.pyplot as plt
import numpy as np

def add_value_labels(ax, spacing=5):
    """Add labels to the end of each bar in a bar chart.

    Arguments:
        ax (matplotlib.axes.Axes): The matplotlib object containing the axes
            of the plot to annotate.
        spacing (int): The distance between the labels and the bars.
    """

    # For each bar: Place a label
    for rect in ax.patches:
        # Get X and Y placement of label from rect.
        y_value = rect.get_height()
        x_value = rect.get_x() + rect.get_width() / 2

        # Number of points between bar and label. Change to your liking.
        space = spacing
        # Vertical alignment for positive values
        va = 'bottom'

        # If value of bar is negative: Place label below bar
        if y_value < 0:
            # Invert space to place label below
            space *= -1
            # Vertically align label at top
            va = 'top'

        # Use Y value as label and format number with one decimal place
        label = "{:.1f}".format(y_value)

        # Create annotation
        ax.annotate(
            label,                      # Use `label` as label
            (x_value, y_value),         # Place label at end of the bar
            xytext=(0, space),          # Vertically shift label by `space`
            textcoords="offset points", # Interpret `xytext` as offset in points
            ha='center',                # Horizontally center label
            va=va)                      # Vertically align label differently for

n_groups = 2
rust_folk = (88.55, 331.707)
cpp_folk = (290.0, 435.0)

# create plot

fig, ax = plt.subplots()
index = np.arange(n_groups)
bar_width = 0.35
opacity = 0.8

rects1 = plt.bar(index, rust_folk, bar_width,
                 label='Rust', color='black')

rects2 = plt.bar(index + bar_width, cpp_folk, bar_width,
                 label='C++ (est.)', color='gray')

# for i, v in enumerate(rust_folk):
#     ax.text(v + 0.25, i + 3, str(v))

#plt.xlabel('Person')
plt.ylabel('Throughput (MOps/sec)')
plt.title('Insert and Find Comparison between Rust and C++')
plt.xticks(index + bar_width/2, ('Insert', 'Find'))
plt.legend()
add_value_labels(ax)

plt.tight_layout()
plt.savefig("figs/compare.png")
