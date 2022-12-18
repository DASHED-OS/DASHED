from task_system import *
import csv
import os
'''NO GLOBAL VALUES'''

def get_selection(action, suboptions, to_upper=True, go_back=False):
        """
        param: action (string) - the action that the user
                would like to perform; printed as part of
                the function prompt
        param: suboptions (dictionary) - contains suboptions
                that are listed underneath the function prompt.
        param: to_upper (Boolean) - by default, set to True, so
                the user selection is converted to upper-case.
                If set to False, then the user input is used
                as-is.
        param: go_back (Boolean) - by default, set to False.
                If set to True, then allows the user to select the
                option M to return back to the main menu
        The function displays a submenu for the user to choose from.
        Asks the user to select an option using the input() function.
        Re-prints the submenu if an invalid option is given.
        Prints the confirmation of the selection by retrieving the
        description of the option from the suboptions dictionary.
        returns: the option selection (by default, an upper-case string).
                The selection be a valid key in the suboptions
                or a letter M, if go_back is True.
        """
        print(list_menu)
        print("-"*42)
        selection = action
        if go_back:
                if 'm' in suboptions or 'M' in suboptions:
                        print("Invalid submenu, which contains M as a key.")
                return None
        if selection not in suboptions:
                print(f"::: What would you like to {action.lower()}?")
                selection = input(">>> ")
        for key in suboptions:
                print(f"{key} - {suboptions[key]}")
                if go_back == True:
                        selection = input(f"::: Enter your selection "
                                f"or press 'm' to return to the main menu\n> ")
                else:
                        selection = input("::: Enter your selection\n> ")
                # check if the selection wants to be upper-case
                if to_upper:
                        selection = selection.upper()  # to allow us to input lower- or upper-case letters
                if go_back and selection.upper() == 'M':
                        return 'M'
                else:
                        print(f"You selected |{selection}| to",
                        f"{action.lower()} |{suboptions[selection]}|.")
        return selection


def print_task(task, priority_map, name_only=False): # FIXME: check the arguments, add a parameter for the task's index
        """
        param: task (dict) - a dictionary object that is expected
                to have the following string keys:
        - "name": a string with the task's name
        - "info": a string with the task's details/description
                (the field is not displayed if the value is empty)
        - "priority": an integer, representing the task's priority
        (defined by a dictionary priority_map)
        - "duedate": a valid date-string in the US date format: <MM>/<DD>/<YEAR>
                (displayed as a written-out date)
        - "done": a string representing whether a task is completed or not

        param: priority_map (dict) - a dictionary object that is expected
                to have the integer keys that correspond to the "priority"
                values stored in the task; the stored value is displayed for the
                priority field, instead of the numeric value.
        param: name_only (Boolean) - by default, set to False.
        If True, then only the name of the task is printed.
        Otherwise, displays the formatted task fields

        returns: None; only prints the task values

        Helper functions:
        - get_written_date() to display the 'duedate' field
        """
        return None


def print_tasks(all_tasks, priority_map, name_only=False,
                show_idx=False, start_idx=0, completed="all"):
        """
        param: task_list (list) - a list containing dictionaries with
                the task data
        param: priority_map (dict) - a dictionary object that is expected
                to have the integer keys that correspond to the "priority"
                values stored in the task; the stored value is displayed
                for the priority field, instead of the numeric value.
        param: name_only (Boolean) - by default, set to False.
                If False, then only the name of the task is printed.
                Otherwise, displays the formatted task fields.
                Passed as an argument into the helper function.
        param: show_idx (Boolean) - by default, set to False.
                If False, then the index of the task is not displayed.
                Otherwise, displays the "{idx + start_idx}." before the
                task name.
        param: start_idx (int) - by default, set to 0;
                an expected starting value for idx that
                gets displayed for the first task, if show_idx is True.
        param: completed (str) - by default, set to "all".
                By default, prints all tasks, regardless of their
                completion status ("done" field status).
                Otherwise, it is set to one of the possible task's "done"
                field's values in order to display only the tasks with
                that completion status.
        returns: None; only prints the task values from the task_list
        Helper functions:
        - print_task() to print individual tasks
        """
        print("-"*42)
        for i in all_tasks: # go through all tasks in the list
                if show_idx: # if the index of the task needs to be displayed
                        print(f"{...}.", end=" ")
                if completed == "all":
                        print_task(all_tasks, priority_map, name_only)
                elif ... == completed:
                        print_task(all_tasks, priority_map, name_only)
        print("-"*42)
        return None


def get_written_date(date_list):
        """
        param: date_str (str) - a string in the US date format: <MM>/<DD>/<YEAR>
        (displayed as a written-out date)
        returns: a string with the written-out date
        """
        month_names = {
        1: "January",
        2: "February",
        3: "March",
        4: "April",
        5: "May",
        6: "June",
        7: "July",
        8: "August",
        9: "September",
        10: "October",
        11: "November",
        12: "December",
        }
        # def info_str(date_str, "date"):
        #         return info_str
        # # Return the date string in written format
        # return info_str(date_str, "date")
        return None

def is_valid_name(name):
        """
        param: name (str) - a string with the task's name.
        This function checks whether the name is valid for the task.
        If it is, between 3 and 25 characters (inclusive of both)
        the "info" field can be empty and has no specific requirements,
        so we do not need to validate it.
        returns: a Boolean True if the text is of the valid length; False, otherwise
        """
        # ⚠️ Pay close attention to the number of parameters and their types:
        # each validation function first checks that the input parameters are of the correct type.
        # Make sure that you use the type() function!
        if name.isalpha() and len(name) >= 3 and len(name) <= 25:
                return True
        else:
                return False

def is_valid_priority(priority, priority_map):
        # ⚠️ Pay close attention to the number of parameters and their types:
        # each validation function first checks that the input parameters are of the correct type.
        # Make sure that you use the type() function!
        """
        param: priority (int) - an integer representing the task's priority
        param: priority_map (dict) - a dictionary object that is expected
        This function checks whether the priority is valid for the task.
        returns: a Boolean True if the priority is in the priority_map; False, otherwise
        """
        if priority in priority_map:
                str.isdigit(priority)
                return True
        else:
                return False

def is_valid_date(date):
        """
        param: date (datetime.date) - a datetime.date object representing the task's due date
        returns: a Boolean True if the date is valid; False, otherwise
        """
        if date is get_written_date(date):
                return True
        else:
                return False

def is_valid_completion(completion):
        """
        param: completion (str) - a string representing the task's completion status
        returns: a Boolean True if the completion status is valid; False, otherwise
        """
        if completion == "yes" or completion == "no":
                return True
        else:
                return False

def is_valid_index(idx, in_list, start_idx = 0):
        """
        param: idx (str) - a string that is expected to
                contain an integer index to validate
        param: in_list - a list that the idx indexes
        param: start_idx (int) - by default, set to 0;
                an expected starting value for idx that
                gets subtracted from idx for 0-based indexing
        The function checks if the input string contains
        only digits and verifies that (idx - start_idx) is >= 0,
        which allows to retrieve an element from in_list.
        returns:
        - True, if idx is a numeric index >= start_idx
        that can retrieve an element from in_list.
        - False if idx is not a string that represents an
        integer value, if int(idx) is < start_idx,
        or if it exceeds the size of in_list.
        """
        if idx.isdigit() and int(idx) >= start_idx and int(idx) < len(in_list):
                return True
        else:
                return False

def update_task(info_list, idx, priority_map, field_key, field_info, start_idx = 0):
        """
        param: info_list - a list that contains task dictionaries
        param: idx (str) - a string that is expected to contain an integer
                index of an item in the input list
        param: start_idx (int) - by default is set to 0;
                an expected starting value for idx that gets subtracted
                from idx for 0-based indexing
        param: priority_map (dict) - a dictionary that contains the mapping
                between the integer priority value (key) to its representation
                (e.g., key 1 might map to the priority value "Highest" or "Low")
                Needed if "field_key" is "priority" to validate its value.
        param: field_key (string) - a text expected to contain the name
                of a key in the info_list[idx] dictionary whose value needs to
                be updated with the value from field_info
        param: field_info (string) - a text expected to contain the value
                to validate and with which to update the dictionary field
                info_list[idx][field_key]. The string gets stripped of the
                whitespace and gets converted to the correct type, depending
                on the expected type of the field_key.
        The function first calls one of its helper functions
        to validate the idx and the provided field.
        If validation succeeds, the function proceeds with the update.
        return:
        If info_list is empty, return 0.
        If the idx is invalid, return -1.
        If the field_key is invalid, return -2.
        If validation passes, return the dictionary info_list[idx].
        Otherwise, return the field_key.
        Helper functions:
        The function calls the following helper functions:
        - is_valid_index()
        Depending on the field_key, it also calls:
        - is_valid_name()
        - is_valid_priority()
        - is_valid_date()
        - is_valid_completion()
        """
        for idx, info in enumerate(info_list):
                if is_valid_index(idx, info_list, start_idx):
                        if field_key == "name":
                                if is_valid_name(field_info):
                                        info_list[idx][field_key] = field_info
                                        return info_list[idx]
                                else:
                                        return -2
                        elif field_key == "priority":
                                if is_valid_priority(field_info, priority_map):
                                        info_list[idx][field_key] = field_info
                                        return info_list[idx]
                                else:
                                        return -2
                        elif field_key == "date":
                                if is_valid_date(field_info):
                                        info_list[idx][field_key] = field_info
                                        return info_list[idx]
                                else:
                                        return -2
                        elif field_key == "completion":
                                return -2
                        else:
                                return -2
                else:
                        return -2
        return 0

def delete_item(in_list, idx, start_idx = 0):
        """
        param: in_list - a list from which to remove an item
        param: idx (str) - a string that is expected to
                contain a representation of an integer index
                of an item in the provided list
        param: start_idx (int) - by default, set to 0;
                an expected starting value for idx that
                gets subtracted from idx for 0-based indexing
        The function first checks if the input list is empty.
        The function then calls is_valid_index() to verify
        that the provided index idx is a valid positive
        index that can access an element from info_list.
        On success, the function saves the item from info_list
        and returns it after it is deleted from in_list.
        returns:
        If the input list is empty, return 0.
        If idx is not of type string or start_idx is not an int, return None.
        If is_valid_index() returns False, return -1.
        Otherwise, on success, the function returns the element
        that was just removed from the input list.
        Helper functions:
        - is_valid_index()
        """
        if len(in_list) == 0:
                return 0
        elif not isinstance(idx, str) or not isinstance(start_idx, int):
                return None
        elif not is_valid_index(idx, in_list, start_idx):
                return -1
        else:
                item = in_list.pop(int(idx) - start_idx)
                return item

def get_new_task(all_tasks, priority_map):
        """
        param: None (list) - a list containing dictionaries with
        param: None (dict) - a dictionary that contains the mapping between the integer priority value (key) to its representation (value)
        The function returns different types of values, depending on whether it succeeds or fails.
        The function expects the first parameter to be a list with 5 strings.
        If the size of the list is not correct, then the function returns the integer size of the provided list.
        E.g., calling get_new_task() with an empty list as its first argument should return 0 and so on.
        If any of the elements on the list are not of type string, the get_new_task() returns a tuple with ("type", <value>),
        where the <value> is substituted with the first corresponding value from the list that was not a string.
        If the size of the list is correct, the function calls the helper functions to validate the fields.
        If the validation succeeds, returns a new dictionary with the task keys set to the provided parameters
        (stripped of whitespace and converted to the proper type, if necessary).
        """
        # Validation functions must be implemented and check that the type of the value is correct.
        # If any of the validation functions fail, returns a tuple with the name of the parameter and the corresponding value/parameter that caused it to fail.
        # The first input parameter is a list of all needed task components/values stored as strings: name, info, priority, date, is_done.
        # {
        # "name": ...,
        # "info": ...,
        # "priority": ...,
        # "duedate": ...,
        # "done": ...
        # }
        while True:
                return None

def load_tasks_from_csv(filename, in_list, priority_map):
        """
        param: filename (str) - A string variable which represents the
                name of the file from which to read the contents.
        param: in_list (list) - A list of task dictionary objects to which
                the tasks read from the provided filename are appended.
                If in_list is not empty, the existing tasks are not dropped.
        param: priority_map (dict) - a dictionary that contains the mapping
                between the integer priority value (key) to its representation
                (e.g., key 1 might map to the priority value "Highest" or "Low")
                Needed by the helper function.
        The function ensures that the last 4 characters of the filename are '.csv'.
        The function requires the `import csv` and `import os`.
        If the file exists, the function will use the `with` statement to open the
        `filename` in read mode. For each row in the csv file, the function will
        proceed to create a new task using the `get_new_task()` function.
        - If the function `get_new_task()` returns a valid task object,
        it gets appended to the end of the `in_list`.
        - If the `get_new_task()` function returns an error, the 1-based
        row index gets recorded and added to the NEW list that is returned.
        E.g., if the file has a single row, and that row has invalid task data,
        the function would return [1] to indicate that the first row caused an
        error; in this case, the `in_list` would not be modified.
        If there is more than one invalid row, they get excluded from the
        in_list and their indices will be appended to the new list that's
        returned.
        returns:
        * -1, if the last 4 characters in `filename` are not '.csv'
        * None, if `filename` does not exist.
        * A new empty list, if the entire file is successfully read from `in_list`.
        * A list that records the 1-based index of invalid rows detected when
        calling get_new_task().
        Helper functions:
        - get_new_task()
        """
        if len(filename) < 4 or filename[-4:] != ".csv":
                return -1
        if not os.path.isfile(filename):
                return None
        with open(filename, 'r') as csvfile:
                reader = csv.reader(csvfile)
                for row in reader:
                        if len(row) == 5:
                                task = get_new_task(row, priority_map)
                                if task is not None:
                                        in_list.append(task)
                        elif len(row) > 5:
                                return -1
                        elif len(row) < 5:
                                return None
                        elif len(row) == 5 and row[0] == "":
                                return None
                        else:
                                return -1
        return in_list

def save_tasks_to_csv(tasks_list, filename):
        """
        param: tasks_list - The list of the tasks stored as dictionaries
        param: filename (str) - A string that ends with '.csv' which represents
                the name of the file to which to save the tasks. This file will
                be created if it is not present, otherwise, it will be overwritten.
        The function ensures that the last 4 characters of the filename are '.csv'.
        The function requires the `import csv`.
        The function will use the `with` statement to open the file `filename`.
        After creating a csv writer object, the function uses a `for` loop
        to loop over every task in the list and creates a new list
        containing only strings - this list is saved into the file by the csv writer
        object. The order of the elements in the list is:
        * `name` field of the task dictionary
        * `info` field of the task dictionary
        * `priority` field of the task as a string
        (i.e, "5" or "3", NOT "Lowest" or "Medium")
        * `duedate` field of the task as written as string
        (i.e, "06/06/2022", NOT "June 6, 2022")
        * `done` field of the task dictionary
        returns:
        -1 if the last 4 characters in `filename` are not '.csv'
        None if we are able to successfully write into `filename`
        """
        if filename[-4:] != '.csv':
                return -1
        with open(filename, 'w') as csvfile:
                writer = csv.writer(csvfile)
                for task in tasks_list:
                        writer.writerow([task['name'], task['info'], task['priority'], task['duedate'], task['done']])
        return None