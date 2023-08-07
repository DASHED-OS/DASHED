import mt4api

# Create an instance of the MT4 API client
client = mt4api.MT4Client()

# Connect to the MT4 server using the provided credentials
client.connect('mt4server.com', 8888, 'username', 'password')

# Get a list of all open positions
positions = client.get_open_positions()
print(positions)

# Get a list of all open orders
orders = client.get_open_orders()
print(orders)

# Place a new order on the market
order = client.place_order('EURUSD', mt4api.ORDER_TYPE_BUY, 0.1, 1.1234) 
print(order)

# Create a python MT4 trading bot
