from accumulation import "/Algo-Trading/Technical_Analysis/Market_Cycle/accumulation.py"

assert trigger_accumulation(stopping_volume, signal_1) == accumulation_trigger
assert process_accumulation("") == accumulation_process
assert begin_filling_warehouse() == warehouse_status

### Test the valid inputs ###
arg1 = stopping_volume
arg2 = signal_1
result = trigger_accumulation(stopping_volume, signal_1)
expected_result = "zero"
print(f"--> trigger_accumulation({arg1, arg2}) returned `{result}`\n")
assert result == expected_result

arg = 5
result = num2str(arg)
expected_result = "five"
print(f"--> num2str({arg}) returned `{result}`\n")
assert result == expected_result

arg = 5
result = num2str(arg)
expected_result = "five"
print(f"--> num2str({arg}) returned `{result}`\n")
assert result == expected_result