from main import *
from functions import *
import _TestRunner

# TODO 0: Test the functions in functions.py
'''This assert will test the print_main_menu() function'''
arg = menu
result = print_main_menu(arg)
expectedResult = ''
print(f'--> print_main_menu({arg}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the get_grades(info_list) function'''
arg = info_list
result = get_grades(arg)
expectedResult = ''
print(f'--> get_grades({arg}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the get_list_avg(num_list) function'''
arg = num_list
result = get_list_avg(arg)
expectedResult = ''
print(f'--> get_list_avg({arg}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the get_total_grade(info_list, show_steps = False) function'''
arg1 = info_list
arg2 = show_steps
result = get_total_grade(arg1, arg2)
expectedResult = ''
print(f'--> get_total_grade({arg1, arg2}) returned `{result}')
assert result == expectedResult

'''This assert will test the delete_item(info_list, idx, start_idx) function'''
arg1 = info_list
arg2 = idx
arg3 = start_idx
result = delete_item(arg1, arg2, arg3)
expectedResult = ''
print(f'--> delete_item({arg1, arg2, arg3}) returned `{result}` \n')
assert result == expectedResult

# TODO 1: Test the functions in main.py
'''This assert will test the update_category() function'''
arg1 = info_list
arg2 = idx
arg3 = info_str
result = update_category(arg1, arg2, arg3)
expectedResult = ''
print(f'--> update_category({arg1, arg2, arg3}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the create_category() function'''
arg = info_str
result = create_category(arg)
expectedResult = ''
print(f'--> create_category({arg}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the is_valid_index(idx, in_list, start_idx) function'''
arg1 = idx
arg2 = in_list
arg3 = start_idx
result = is_valid_index(arg1, arg2, arg3)
expectedResult = ''
print(f'--> is_valid_index({arg1, arg2, arg3}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the get_selection(action, suboptions) function'''
arg1 = action
arg2 = suboptions
result = get_selection(arg1, arg2)
expectedResult = opt
print(f'--> get_selection({arg1, arg2}) returned `{result}` \n')
assert result == expectedResult

'''This assert will test the print_grade_info(info_list, show_grades) function'''
arg1 = info_list
arg2 = show_grades
result = print_grade_info(info_list, show_grades = False)
expectedResult = ''
print(f'--> print_grade_info({arg1, arg2}) returned `{result}')
assert result == expectedResult