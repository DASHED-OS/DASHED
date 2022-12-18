import math
import os
import sys
import subprocess
'''NOTES = [Operator, Subprocess Operator, Automate Management, Power Automate]'''

def operator():
    os.walk('/home/james/Desktop/Python/packages/performance/auto')
    return None
def subprocess_operator():
    subprocess.call(['ls', '-l', '-a'])   # -l for long listing format and -a for all files and directories
    subprocess.call(['ls', '-l', '-a'], stdout=subprocess.PIPE)
    return None

tasks = list(input('Enter tasks: '))
grades = list(input('Enter grades: '))
def automate_management():
    for (index, task) in enumerate(tasks):
        print('\n')
        print(f'{index + 1} - {task["title"]}')
        print(f'{task["description"]}')
        print(f'{task["link"]}')
        print(f'{task["date"]}')
        print(f'{task["time"]}')
        print(f'{task["location"]}')
        print(f'{task["category"]}')
        print(f'{task["priority"]}')
        print(f'{task["status"]}')
        print(f'{task["id"]}')
        print(f'{task["created_at"]}')
        print(f'{task["updated_at"]}')
        print(f'{task["deleted_at"]}')
        print(f'{task["created_by"]}')
        print(f'{task["updated_by"]}')
        print(f'{task["deleted_by"]}')
        print(f'{task["assigned_to"]}')
        if type(task["assigned_to"]) is list:
            for (index, assignee) in enumerate(task["assigned_to"]):
                print(f'{index + 1} - {assignee["name"]}')
        else:
            continue
    print(f'{task["assigned_by"]}')
    for (index, assignee) in enumerate(task["assigned_to"]):
        print(f'{index + 1} - {assignee["name"]}')
    print('\n')
    for (index, grade) in enumerate(grades):
        print(f'{index + 1} - {grade["name"]}')
        print(f'{grade["description"]}')
        print(f'{grade["link"]}')
        print(f'{grade["date"]}')
        print(f'{grade["time"]}')
        print(f'{grade["location"]}')
        print(f'{grade["category"]}')
        print(f'{grade["priority"]}')
        print(f'{grade["status"]}')
        print(f'{grade["id"]}')
        print(f'{grade["created_at"]}')
        print(f'{grade["updated_at"]}')
        print(f'{grade["deleted_at"]}')
        print(f'{grade["created_by"]}')
        print(f'{grade["updated_by"]}')
        print(f'{grade["deleted_by"]}')
        print(f'{grade["assigned_to"]}')
        if type(grade["assigned_to"]) is list:
            for (index, assignee) in enumerate(grade["assigned_to"]):
                print(f'{index + 1} - {assignee["name"]}')
        else:
            continue
        print(f'{grade["assigned_by"]}')
    return None

def power_automate():
    def automations():
        return None
    automations = automate_management()
    return None

