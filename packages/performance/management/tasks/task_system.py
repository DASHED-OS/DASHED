from task_functions import *
'''NO GLOBAL VALUES'''

the_menu = {"L": "List", "A": "Add", "U": "Update", "D": "Delete", "S": "Save the data to file",
            "R": "Restore data from file", "Q": "Quit this program"}  # dictionary of menu options # FIXME: automate_management()

all_tasks = [
    {
        "name": "Call XYZ",
        "info": "",
        "priority": 3,
        "duedate": "05/28/2022",
        "done" : "yes"
    },
    {
        "name": "Finish checkpoint 1 for CSW8",
        "info": "Submit to Gradescope",
        "priority": 5,
        "duedate": '06/02/2022',
        "done": 'no'
    },
    {
        "name": "Finish checkpoint 2 for CSW8",
        "info": "Implement the new functions",
        "priority": 5,
        "duedate": '06/05/2022',
        "done": 'no'
    }

]

# list_menu will contain the "List" menu suboptions
list_menu = {
    "A": "all tasks",
    "C": "completed tasks",
    "I": "incomplete tasks"
}

# priority_map will contain the mapping of the integer priority values to their textual interpretation
priority_map = {
    1: "Lowest",
    2: "Low",
    3: "Medium",
    4: "High",
    5: "Highest"
}

while True:
    print(the_menu)
    opt = input("::: Enter a menu option\n> ")
    opt = opt.upper()  # to allow us to input lower- or upper-case letters
    if opt == 'L':
        if all_tasks == []:
            print("WARNING: There is nothing to display!")
            # Pause before going back to the main menu
            input("::: Press Enter to continue")
            continue
        subopt = get_selection(the_menu[opt], list_menu) # FIXME: this is a placeholder for the function
        input("::: Enter a menu option\n> ")
        # if subopt == 'A':
        #     print_tasks(all_tasks, priority_map) # FIXME: this is a placeholder for the function
        # elif subopt == 'C':
        #     print_tasks(all_tasks, priority_map, completed='yes') # FIXME: this is a placeholder for the function
        # elif subopt == 'I':
        #     print_tasks(all_tasks, priority_map, completed='no') # FIXME: this is a placeholder for the function
#     elif opt == 'A':
#         continue_action = 'y'
#         while continue_action == 'y':
#             print("::: Enter each required field, separated by commas.")
#             print("::: name, info, priority, MM/DD/YYYY, is task done? (yes/no)")
#             task = input("> ") # TODO: get and process the data into a list
#             task = task.split(',')
#             result = get_new_task(all_tasks, priority_map) # TODO: attempt to create a new task
#             if type(result) == dict:
#                 all_tasks = all_tasks.append(result) # TODO: add a new task to the list of tasks
#                 print(f"Successfully added a new task!")
#                 print_task(result, priority_map)
#             elif type(result) == int:
#                 print(f"WARNING: invalid number of fields!")
#                 print(f"You provided {result}, instead of the expected 5.\n")
#             else:
#                 print(f"WARNING: invalid task field: {result}\n")
#             print("::: Would you like to add another task?", end=" ")
#             continue_action = input("Enter 'y' to continue.\n> ")
#             continue_action = continue_action.lower()
#             # ----------------------------------------------------------------
#     elif opt == 'U':
#         continue_action = 'y'
#         while continue_action == 'y':
#             if ... == []: # TODO: check the return value of get_selection
#                 print("WARNING: There is nothing to update!")
#                 break
#             print("::: Which task would you like to update?")
#             print_tasks(all_tasks, priority_map, name_only = True, show_idx = True, start_idx = 1)
#             print("::: Enter the number corresponding to the task.")
#             user_option = input("> ")
#             if ...: # TODO: check the return value of get_selection
#                 ... # TODO: convert the index appropriately to account for the start_idx = 1
#                 subopt = get_selection("update", all_tasks[...], to_upper = False, go_back = True)
#                 if subopt == 'M': # if the user changed their mind
#                     break
#                 print(f"::: Enter a new value for the field |{...}|") # TODO: get the error message from the function
#                 field_info = input("> ")
#                 result = update_task(all_tasks, user_option, priority_map, subopt, field_info, start_idx = 1)
#                 if type(result) == dict:
#                     print(f"Successfully updated the field |{...}|:")  # TODO: get the error message from the function
#                     print_task(result, ...)  # TODO: Call the function with appropriate inputs
#                 else: # update_task() returned an error
#                     print(f"WARNING: invalid information for the field |{...}|!")  # TODO: get the error message
#                     print(f"The task was not updated.")
#             else: # is_valid_index() returned False
#                 print(f"WARNING: |{...}| is an invalid task number!")  # TODO: get the index from the user
#             print("::: Would you like to update another task?", end=" ")
#             continue_action = input("Enter 'y' to continue.\n> ")
#             continue_action = continue_action.lower()
#         # ----------------------------------------------------------------
#     elif opt == 'S':
#         continue_action = ...
#         while continue_action == 'y':
#             print("::: Enter the filename ending with '.csv'.")
#             filename = input("> ")
#             result = save_tasks_to_csv(tasks_list, filename) # TODO: Call the function with appropriate inputs and capture the output
#             if result == -1: # TODO: check the return value of the function
#                 print(f"WARNING: |{...}| is an invalid file name!") # TODO: get the error message from the function
#                 print("::: Would you like to try again?", end=" ")
#                 continue_action = input("::: Enter 'y' to try again.\n> ")clea
#             else:
#                 print(f"Successfully stored all the tasks to |{...}|")
#                 print("::: Would you like to save another file?", end=" ")
#     elif opt == 'Q' or 'q':
#         print("Goodbye!\n")
#         break  # exit the main `while` loop
#     #--------------------------------------------------------------------------
#     else:
        # print(f"WARNING: {opt} is an invalid menu option.\n")
#     input("::: Press Enter to continue")
#     print(f"You selected option {opt} to > {the_menu[opt]}.")

# print("Have a nice day!")
