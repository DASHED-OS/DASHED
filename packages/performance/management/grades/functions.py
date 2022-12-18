import math
from main import * # import the main function

menu = {"T": "Total grade", "L": "List", "A": "Add", "U": "Update", "D": "Delete",
        "S": "Save the data", "R": "Restore data from file", "Q": "Quit this program"}  # TODO 1: add the options from the instructions
opt = None

def print_main_menu(menu):
    """
    Given a dictionary with the menu,
    prints the keys and values as the
    formatted options.
    Adds additional prints for decoration
    and outputs a question
    "What would you like to do?"
    """
    print(menu) # TODO: Use the menu to list out the options instead
    opt = str(input("::: Enter"))
    opt = action.upper()
    print(f'You chose the {suboptions[opt]} option')
    i = 1
    # while enumerate(menu) and len(menu) == True :
    #     print(f'What would you like to do?')
    #     while opt in suboptions:
    #         if opt not in suboptions: #Base Case
    #             print("Invalid action")
    #             print(suboptions)
    #             selection = input("Enter an action (T, L, A, U, D, S, R, Q): ")
    #         elif opt == 'T': # Total grade
    #             info_list = get_total_grade(all_grades, True)
    #             print("Current total is {total:.2f}")
    #         elif suboptions == 'L': # List
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'A': # Add
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'U': # Update
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'D': # Delete
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'S': # Save the data
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'R': # Restore data from file
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #         elif suboptions == 'Q': # Quit this program
    #             selection = suboptions[selection]
    #             print(print_grade_info(info_list, show_grades = False))
    #             continue
    #         else:
    #             print(f"{action} is not a valid option.")
    #             print(f"Please enter a valid option.")
    #             continue  # continue to the next iteration of the loop
    return print(f"\n{'-'*40}\n{' '*20}MENU\n{'-'*40}\n{' '*20}")

num_list = [{}, {}, {}]
def get_list_avg(num_list, show_steps):
    """
    param: num_list - a nested list of lists
    The function expects that each nested lists contains
    only the numeric values.
    return:
    The function returns a new list that contains the
    average value of each sub-list in num_list.
    If the input is an empty list, the function returns
    an empty list.
    """
    avg_sum = 0
    info_list_0 = get_grades(info_list)
    avg_grades = get_list_avg(info_list_0)
    for category_position in enumerate(info_list):  # enumerate()
    #     get the category average value from avg_grades (at the same index as it is in info_list)
    #     get the category weight from the info_list
    #   compute the category grade: (category average value * (category weight / 100))
    #   add the category grade to avg_sum
        if show_steps >= len(show_steps):
            print(f"{...} average {...:.2f}%", end =" ") # see the example in the instructions
            print(f"{...} / {...}")
    return avg_sum / len(info_list) # len(info_list)

info_list = list(input())
def extract_list(nested_list):
    # Create a function to extract a list from a nested list that contains dictionaries
    # Read and follow function documentation
    # Use for loops to iterate over the items in a list, potentially using enumerate()
    # Use append() to add elements into a new list
    # Use list methods sum() and len() to compute the average of a list
    # Write a new function that uses previously-written functions as helpers
    # Create and run assert statements to check the function correctness
    action = nested_list
    action = action.append()
    extracted_list = nested_list + action
    return info_list [0] # TODO: return extracted_list


def get_grades(info_list):
    """
    param:
    info_list - a list that contains dictionaries
    Each dictionary in the info_list is supposed to have
    the following string keys:
    - category - the name of each grade category (string)
    - weight - the percentage weight of the category (float)
    - grades - a list of numeric grades for each assignment
    If no grades are stored for a category, the "grades"
    item will store an empty list.
    return:
    The function returns a new (nested) list, which contains
    the extracted grades for each category.
    If the input is an empty list, the function returns
    an empty list.
    """
    while len(info_list) == 0:
        try:
            #...Normal code that might produce errors
            info = info_list [0]
        except:
            #...Exception code
            info_list.append
    extract_list = info_list [0]
    grades_per_category = info_list[0]
    return grades_per_category # TODO: add the return statement

def get_total_grade(info_list, show_steps=False):
    """
    param: info_list - a list that contains dictionaries
    param: show_steps - a Boolean flag (False by default)
            controls whether the function outputs intermediate
            steps, outputting the average scores.

    Each dictionary in the info_list is supposed to have
    the following string keys:
    - category - the name of each grade category (string)
    - weight - the percentage weight of the category (float)
    - grades - a list of numeric grades for each assignment
    If no grades are stored for a category, the "grades"
    item will store an empty list.

    return:
    The function returns a floating-point value that results from
    summing up the product of the weight of each category with the
    average value of each "grades" list.
    The function returns 0 if the info_list is empty.

    Helper functions:
    The function calls the following helper functions:
    - get_grades() to extract the grades from info_list
    - get_list_avg() to compute the average score for each
    extracted grade list
    """
    def get_grades():
        extracted_list = info_list [0]
        grades_per_category = info_list[0]
        return grades_per_category
    def get_list_avg():
        avg_grades = get_grades()
        avg_sum = 0
        for category_position in enumerate(info_list):  # enumerate()
            #     get the category average value from avg_grades (at the same index as it is in info_list)
            #     get the category weight from the info_list
            #   compute the category grade: (category average value * (category weight / 100))
            #   add the category grade to avg_sum
            if show_steps >= len(show_steps):
                print(f"{...} average {...:.2f}%", end =" ")
                print(f"{...} / {...}")
        return avg_sum
    return info_list [0] # TODO: add the return statement

# carefully reading the provided function documentation to know what to implement
# using your previously-written functions to help implement new functionality
# using Boolean value as a flag to know whether to run through the loop again
# working with dictionaries stored inside a list (a nested list) - using indexing to retrieve specific values
# removing items from a list
# formatting the output according to the specifications
# writing test cases using assert statements
def delete_item(info_list, idx, start_idx):
    """
    param: info_list - a list from which to remove
            an item
    param: idx (str) - a string that is expected to
            contain an integer index of an item in
            the in_list
    param: start_idx (int) - an expected starting
            value for idx (default is 0); gets
            subtracted from idx for 0-based indexing

    The function first checks if info_list is empty.
    The function then calls is_valid_index() to verify
    that the provided index idx is a valid positive
    index that can access an element from info_list.
    On success, the function saves the item from info_list
    and returns it after it is deleted from info_list.

    returns:
    If info_list is empty, return 0.
    If is_valid_index() returns False, return -1.
    Otherwise, on success, the function returns the element
    that was just removed from info_list.

    Helper functions:
    - is_valid_index()
    """
    # def is_valid_index(info_list):
    #     info_list = {}
    #     validation = False
    #     for info in info_list:is_valid_index()
    #     validation = is_valid_index()
    #     return validation
    # return
    def is_valid_index():
        if idx >= 0:
            return True
        else:
            return False
    the_menu = {}  # TODO: add the options from the instructions
    opt = None
    while True:
        print_main_menu(menu) # TODO: uncomment, define the function, and call with the menu as an argument
        print("::: Enter an option")
        opt = input("> ")
        if opt == 'Q' or 'q':  # TODO: make Q or q quit the program
            print("Goodbye!\n")
            break  # exit the main `while` loop
        break  # exit the main `while` loop
    return True # return True to indicate that the program should continue # TODO: add the return statement

# # TODO: Check that these assert functions are covered
# only_grades = [[100.0, 100.0, 100.0, 100.0, 100.0, 0.0, 95.0], [100.0, 100.0, 98.0, 95.0, 0.0, 100.0], [100.0, 100.0, 100.0, 5.0, 0.0, 70.0], [], []]
# assert get_grades(all_grades) == only_grades
# only_grades_test = [
#     { "grades" : [100.0, 100.0, 100.0, 100.0, 100.0, 0.0, 95.0] },
#     { "grades" : [100.0, 100.0, 98.0, 95.0, 0.0, 100.0] },
#     { "grades" : [100.0, 100.0, 100.0, 5.0, 0.0, 70.0] },
#     { "grades" : [] },
#     { "grades" : [] }
# ] # same as all_grades but without the other keys
# assert get_grades(only_grades_test) == only_grades
# avg_grades = [85.0, 82.16666666666667, 62.5, 0, 0]
# assert get_list_avg(only_grades) == avg_grades
# assert get_total_grade(all_grades) - 32.20 <= 0.001
# assert int(get_total_grade([])) == 0