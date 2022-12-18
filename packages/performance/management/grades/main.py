from functions import *

# carefully reading the provided function documentation to know what to implement
# using your previously-written functions to help implement new functionality
# using Boolean value as a flag to know whether to run through the loop again
# working with dictionaries stored inside a list (a nested list) - using indexing to update specific values
# creating a new dictionary and its elements and indexing it to retrieve its values
# formatting the output according to the specifications
# writing test cases using assert statements

def update_category(info_list, idx, info_str):
    """
    Geographically Optimize the Code
    param: info_list - a list that contains dictionaries
    param: idx (int) - a valid positive zero-based index
            that retrieves a dictionary from in_list
    param: info_str (string) is expected to contain two
            space-separated items:
            a category name (ideally, a single word)
            and its numeric grade weight.

    The function first calls its helper function
    create_category() and stores its return value.
    If create_category() successfully returned a new
    dictionary with the info provided in the info_str,
    then the function proceeds to update info_list[idx]
    with the information from the returned dictionary.
    The function does not update the values for "grades".
    return:
    If create_category() validation passes,
    return info_list[idx].
    Otherwise, return the error from create_category().

    Errors:
    See create_category()

    Helper functions:
    The function calls the following helper functions:
    - create_category() to split the input string and validate
    its components.

    Known vulnerability:
    The function assumes that the caller provided
    a valid info_list index, so it does not run the validation
    of idx.
    """
    validation = create_category(info_str)
    if type(validation) == dict:
        return info_list[idx]
    else:
        return validation
# carefully reading the provided function documentation to know what to implement
# using your previously-written functions to help implement new functionality
# using Boolean value as a flag to know whether to run through the loop again
# working with dictionaries stored inside a list (a nested list) - using append()
# creating a new dictionary and its elements
# formatting the output according to the specifications
# writing test cases using assert statements


def create_category(info_str):
    """
    param: info_str (string) is expected to contain two
            space-separated items:
            a category name (ideally, a single word)
            and its numeric grade weight.
    The function splits the input string and validates its
    components (see the errors and helper functions).

    return:
    If the validation passes, return a new dictionary with
    the following string keys are assigned to the values that
    were extracted from the info_str parameter:
    - category - the name of each grade category (string)
    - weight - the percentage weight of the category (float)
    - grades - an empty list (for storing future grades)
    Example: an input string "Quiz 25" results in a dictionary
    { 'category': 'Quiz', 'weight': 25.0, 'grades': [] }

    Errors:
    If any of the errors occur, a new dictionary is not created.
    If info_str.split() does not result in a two-element list,
    then return -2.
    If a category name is less than 2 characters or contains a
    comma, then return -1.
    If the weight (the percentage) in info_str is not numeric
    (int or float), return 0.

    Helper functions:
    The function calls the following helper functions:
    - is_num() to verify that the weight is a numeric value.
    """
    def is_num(info_str):
        """
        param: info_str (string)
        The function returns True if the input string is
        numeric (int or float), and False otherwise.
        """
        return info_str.isdigit()
    # using Boolean value as a flag to know whether to run through the loop again
    # working with dictionaries stored inside a list (a nested list) - using append()
    # creating a new dictionary and its elements
    # formatting the output according to the specifications
    category = info_str.split()
    if len(category) != 2:
        return -2
    elif len(category[0]) < 2 or ',' in category[0]:
        return -1
    elif not is_num(category[1]):
        return 0
    info_list = {'category': category[0], 'weight': float(category[1]), 'grades': []}
    return info_list


def is_valid_index(idx, in_list, start_idx=0):
    """
    param: idx (str) - a string that is expected to
            contain an integer index to validate
    param: in_list - a list that the idx indexes
    param: start_idx (int) - an expected starting
            value for idx (default is 0); gets
            subtracted from idx for 0-based indexing

    The function checks if the input string contains
    only digits and verifies that the provided index
    idx is a valid positive index that can retrieve
    an element from in_list.

    returns:
    - True, if idx is a positive numeric index
    that can retrieve an element from in_list.
    - False if idx is not an integer value, is negative
    or exceeds the size of in_list.
    """
    # writing a function with the default parameter value
    # using Boolean value as a flag for whether printing needs to occur
    # working with dictionaries stored inside a list (a nested list)
    # indexing and printing dictionary entries (potentially using enumerate())
    # formatting the output according to the specifications
    if (idx - start_idx) >= 0 and (idx - start_idx) < len(in_list) and idx.isdigit():
        return True
    else:
        return False


action = str(input("Enter an action (T, L, A, U, D, S, R, Q): "))
suboptions = {"T": "Total grade", "L": "List", "A": "Add", "U": "Update", "D": "Delete",
              "S": "Save the data", "R": "Restore data from file", "Q": "Quit this program"} # TODO: this is the menu


def get_selection(action, suboptions):
    """
    param: action (string) - the action that the user
            would like to perform; printed as part of
            the function prompt
    param: suboptions (dictionary) - contains suboptions
            that are listed underneath the function prompt.
            The keys are assumed to be in upper-case.

    The function displays a submenu for the user to choose from.
    Asks for user to select an option using the input() function.
    Re-prints the submenu if an invalid option is given.
    Prints the confirmation of the selection by retrieving the
    description of the option from the suboptions dictionary.

    returns: the option selection as an upper-case string
            (should be a valid key in the suboptions)
    """
    print(f"What would you like to do?")
    opt = str(action.upper())
    print(suboptions)
    for (index, value) in enumerate(suboptions):
        if opt in suboptions:
            print(f"You selected {suboptions[opt]}")
            break
        else:
            print(f"Please select an option from the existing menu")
            continue  # continue to the next iteration of the loop
    return opt

info_list = [{"category": "", "weight": 0.0, "grades": []}, {"category": "", "weight": 0.0, "grades": []}]

def print_grade_info(info_list, show_grades=True):
    """
    param: info_list - a list that contains dictionaries
    param: show_grades - a Boolean flag (True by default)
            controls whether the function outputs the grades
            for each assignment; if set to False, only displays
            category information
    Each dictionary in the info_list is supposed to have
    the following string keys:
    - category - the name of each grade category (string)
    - weight - the percentage weight of the category (float)
    - grades - a list of numeric grades for each assignment
    If no grades are stored for a category, the "grades"
    item will store an empty list.
    The function prints the grade information in the
    following format (if show_grades is True):
        {N} - {Category name} ({category percentage}%)
        {list of grades}
        ---
    where N is each category's ordinal number
    (note that numbering starts at 1)
    Example:
    for an input list with a single category
    [ { "category": "PA", "weight" : 5,
        "grades" : [100.0, 100.0]} ]
    the function outputs the following:
        1 - PA (5%)
        [100.0, 100.0]
        ---
    The function prints a single line with "---"
    if the provided list is empty.
    return:
    The function does not return anything.
    """
    info_str = str(info_list) # FIXME: this is a string
    for i in enumerate(info_list):
        if show_grades == True: # TODO: Store grades in the dict
            info_str = f"[{N} - {category_name} ({category_percentage} %) {list_of_grades}]" # TODO: Print the information in this format
            print(info_str)
        elif grades == []:
            print("---")
        else:
            continue
        show_grades == False
