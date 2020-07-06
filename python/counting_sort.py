def counting_sort(list):
	
	if len(list) == 0:
		return list
	
	items = {} # a hashmap we'll use to store all of the items
	range_max = list[0] # current maximum of the list
	range_min = list[0]  # current minimum of the list

	for item in list:
		if item in items:
			items[item] = items[item] + 1
		else:
			items[item] = 1

		if item < range_min: range_min = item
		if item > range_max: range_max = item

	sorted_list = [] # new sorted list
	current = range_min # start adding elements to the sorted list from the min

	# add elements to the sorted_list
	while current <= range_max:
		if current in items: # only add if the current is in the dictionary
			for _i in range(items[current]): # add it the right amount of times
				sorted_list.append(current)
		current += 1 # go to the next number

	return sorted_list
