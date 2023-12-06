import requests
import json
import hmac
import hashlib
import time

# Set up API credentials
API_KEY = 'your_api_key'
API_SECRET = 'your_api_secret'

# Set up API endpoints
API_BASE_URL = 'https://api.cryptohopper.com/v1'
API_MARKET_URL = 'https://api.binance.com/api/v3'

# Set up strategy parameters
PAIR = 'BTCUSDT'
BUY_THRESHOLD = 0.05
SELL_THRESHOLD = 0.05

# Define API request function
def make_request(endpoint, params=None, method='GET'):
    # Set up request headers
    headers = {
        'Content-Type': 'application/json',
        'User-Agent': 'Cryptohopper API Client'
    }

    # Set up request parameters
    if params is None:
        params = {}
    params['timestamp'] = int(time.time() * 1000)

    # Generate signature
    query_string = '&'.join([f'{k}={v}' for k, v in params.items()])
    signature = hmac.new(API_SECRET.encode(), query_string.encode(), hashlib.sha256).hexdigest()

    # Add signature to request parameters
    params['signature'] = signature

    # Make API request
    if method == 'GET':
        response = requests.get(f'{API_BASE_URL}/{endpoint}', headers=headers, params=params)
    elif method == 'POST':
        response = requests.post(f'{API_BASE_URL}/{endpoint}', headers=headers, json=params)
    else:
        raise ValueError(f'Invalid HTTP method: {method}')

    # Parse response JSON
    response_json = json.loads(response.text)

    # Check for errors
    if 'error' in response_json:
        raise ValueError(response_json['error'])

    return response_json

# Define function to get current market price
def get_market_price(pair):
    response = requests.get(f'{API_MARKET_URL}/ticker/price', params={'symbol': pair})
    response_json = json.loads(response.text)
    return float(response_json['price'])

# Define function to get current balance
def get_balance():
    response = make_request('user/balance', method='POST')
    return response['data']

# Define function to place buy order
def place_buy_order(pair, amount):
    response = make_request('order/buy', params={'pair': pair, 'amount': amount}, method='POST')
    return response['data']

# Define function to place sell order
def place_sell_order(pair, amount):
    response = make_request('order/sell', params={'pair': pair, 'amount': amount}, method='POST')
    return response['data']

# Define main function
def main():
    # Get current market price
    market_price = get_market_price(PAIR)

    # Get current balance
    balance = get_balance()

    # Calculate buy and sell amounts
    buy_amount = balance['USDT'] * BUY_THRESHOLD / market_price
    sell_amount = balance['BTC'] * SELL_THRESHOLD

    # Place buy order
    buy_order = place_buy_order(PAIR, buy_amount)

    # Place sell order
    sell_order = place_sell_order(PAIR, sell_amount)

    # Print results
    print(f'Bought {buy_order["amount"]} {buy_order["pair"]} at {buy_order["price"]}')
    print(f'Sold {sell_order["amount"]} {sell_order["pair"]} at {sell_order["price"]}')

if __name__ == '__main__':
    main()
