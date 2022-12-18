#Utilities
def candle_spread(candle_high, candle_low):
    candle_spread = candle_high - candle_low,
    return candle_spread

#Calculating setting up triggers based on candles
def trigger_alert_widespread(candle_open, candle_high, candle_low, candle_close, candle_spread):
''' This function will trigger when a widespread candle has been identified on the chart of choice, 
it is also linked to a series of other triggers. Check the alerts.py file for the full directory. '''
    if candle_open => 1/3 * candle_spread:
    alert_widespread = TRUE,
    return alert_widespread

def trigger_alert_narrowspread(candle_open, candle_high, candle_low, candle_close, candle_spread):
''' This function will trigger when a narrowspread candle has been identified on the chart of choice, 
it is also linked to a series of other triggers. Check the alerts.py file for the full directory. '''
    if candle_open = 2/3 * candle_spread:
        alert_narrowspread = TRUE,
    else:
        alert_narrowspread = FALSE,
    return alert_narrowspread

def trigger_alert_hammer(): 
''' This function will trigger when a hammer candle has been identified on the chart of choice, 
it is also linked to a series of other triggers. Check the alerts.py file for the full directory. '''
    if candle_open = hammer
        alert_hammer = TRUE,
    else:
        alert_hammer = FALSE,
    return alert_hammer

def trigger_alert_shooting_star(): 
''' This function will trigger when a hammer candle has been identified on the chart of choice, 
it is also linked to a series of other triggers. Check the alerts.py file for the full directory. '''
    if candle_open = shooting_star
        alert_shooting_star = TRUE,
    else:
        alert_shooting_star = FALSE,
    return alert_shooting_star

if __candle_volume__ = "__main__"

# Grab input from the platform
candle_open = float(input())
candle_high = float(input())
candle_low = float(input())
candle_close = float(input())
candle_spread = candle_spread(candle_high, candle_low)

print(trigger_alert_candle_pattern(candle_open, candle_high, candle_low, candle_close))
assert alert = TRUE
assert candle_spread = candle_high - candle_low
