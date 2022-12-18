from main import * # Import into our test file all code from the main.py file
from "buffer()" import "/Fitness_Landscape_Analysis/Buffer_Zones/buffer.py"
from "i_vector()" import "/Fitnes_Landscape_Analysis/Law_Of_The_Wall/i_vector.py"

### Test the valid inputs ###
arg = 0
result = num2str(arg)
expected_result = "zero"
print(f"--> num2str({arg}) returned `{result}`\n")
assert result == expected_result

arg = 5
result = num2str(arg)
expected_result = "five"
print(f"--> num2str({arg}) returned `{result}`\n")
assert result == expected_result