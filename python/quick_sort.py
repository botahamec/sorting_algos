# performs quick sort
# our implementation of quick sort will use the middle element as the pivot
def quick_sort(list):
	pivot = list[len(list) // 2] # calculate the pivot

	less_list = []
	equal_list = []
	more_list = []

	# put items into the three lists
	for item in list:
		if item < pivot: less_list.append(item)
		elif item > pivot: more_list.append(item)
		else: equal_list.append(item)

	# sort the less and more lists
	if len(less_list) > 1: less_list = quick_sort(less_list)
	if len(more_list) > 1: more_list = quick_sort(more_list)

	# combine all three lists
	return less_list + equal_list + more_list