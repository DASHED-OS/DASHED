import "selling_climax" from "selling_climax.py"
import "stopping_volume" from "stopping_volume.py"
import "signal_"* from "postulate_billions_signals.py"

def signal_accumulation(stopping_volume, signal_1):
    '''This function will process the accumulation trigger using stopping volume signals.'''
    return accumulation_signal



def process_accumulation(accumulation_signal, vol_ma5, vol_ma10, vol_ma20):
    '''
    This function will process the accumulation phrase which takes into account 
    a few different steps. Such as the sign of stopping volume, a test of the supply
    and an overall consolidation in this zone as the warehouse begins and finishes
    filling.

    Arguments:
    accumulation_signal -- Array, signal that began the accumulation, which is based on a set of alerts. 
    vol_ma5 -- Integer, culaculation of the 5 minute Moving Average for Volume.
    '''
    def accumulation_stopping_volume(accumulation_trigger):
        global stopping_volume_signal
        if volume > vol_ma5:
            print('Stopping volume beginning')
        elif volume > vol_ma10:
            volume = volume[vol_ma10]
        else volume >= 0:
            print('Not stopping volume yet')
        while volume > vol_ma20:
            print(stopping_volume_signal)
        return
    def accumulation_testing_supply():
        return 
    def accumulation_get_warehouse_current_supply():
        return
    def accumulation_calculate_goal():
        return
    return accumulation_process

def begin_filling_warehouse():
    '''
    This function start to fill the warehouse with its supplies, which will be the parcels
    that it is designed to hold this time. Based on the signals from the market.
    '''
    #FIXME: #4 Start the buying_signals, pivots, congestion, low-volume tests
    return warehouse_status

def finish_filling_warehouse():
    '''This function will take in the signal for selling_climax and begin to finish filling the warehouse'''
    #FIXME: Take in the selling_climax signal and begin to finish fillinf the warehouse
    return warehouse_status

def trigger_buying_climax():
    '''This function will signal after the closing of the warehouse and bring about the next phase the buying_climax'''
    #FIXME: Srigger the buying_climax
    return buying_climax_signal