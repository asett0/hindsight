from flask import Flask, request
import requests
import os
from datetime import datetime
import json

app = Flask(__name__)

token = os.environ["FINNHUB_KEY"]
base_url = "https://finnhub.io/api/v1"
stock_candle = "/stock/candle"


@app.route("/api/stock/close/daily")
def get_stock_close():
    symbol = request.args.get("symbol")
    day_from = request.args.get("from")
    day_to = request.args.get("to")

    payload = {"symbol": symbol + ".AX", "from": day_from, "to": day_to, "token": token, "resolution": "D"}
    # data  = json.loads()

    return requests.get(base_url + stock_candle, params=payload).json()
 

