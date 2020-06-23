# merges two lists
def merge(first, second):
	list = [] # new sorted list
	i = 0 # index in first list
	j = 0 # index in second list

	# repeat until the end of one of the lists is reached
	while i < len(first) and j < len(second):
		# add next element
		if first[i] < second[j]: # occurs if the first list has the next element
			list.append(first[i]) # adds next element from the first list
			i += 1 # increments the index into the first list
		else: # occurs if the second list has the next element
			list.append(second[j])
			j += 1

	# once the end of one list is reached, add the rest of the other list
	while i < len(first):
		list.append(first[i])
		i += 1
	while j < len(second):
		list.append(second[j])
		j += 1

	return list

def merge_sort(list):
	mid = len(list) // 2 # midpoint of the list
	if len(list) > 1: # stop if the length of the list is only one
		# splits the list in half and sorts each side
		first = merge_sort(list[0:mid])
		second = merge_sort(list[mid:len(list)])
		return merge(first, second) # merges the two halves
	else:
		return list