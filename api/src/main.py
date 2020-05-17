from flask import Flask
import requests
import os

app = Flask(__name__)


def get_daily_url(stock_code):
    return (
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol="
        +stock_code
        +"&apikey="
        +os.environ["ALPHA_ADVANTAGE_API_KEY"]
    )


@app.route("/api/<string:stock_code>")
def get_stock_data(stock_code):
    print(get_daily_url(stock_code))
    return requests.get(get_daily_url(stock_code)).content
 

