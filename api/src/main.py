from flask import Flask

app = Flask(__name__)


@app.route("/api/<string:stock_code>")
def get_stock_data():
    print(f'Code is {stock_code}')
    # return "Hello, World!"

