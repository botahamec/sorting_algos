# swaps two values in a list
def swap(list, first, second):
	temp = list[first]
	list[first] = list[second]
	list[second] = temp

# inserts one element into the list
def insert(list, value):
	index = len(list) # keeps track of the current location of the new value
	list.append(value) # adds the new value to the list

	# repeats until the value is in its proper place
	while index > 0 and list[index - 1] > value:
		swap(list, index, index - 1) # shifts the value farther into the list
		index -= 1


def insertion_sort(list):
	sorted_list = []

	# for each item in the unsorted list, insert it into the sorted list
	for item in list:
		insert(sorted_list, item)

	return sorted_list