from datetime import datetime
from flask import Flask, jsonify, request
from flask_cors import CORS

app = Flask(__name__)

WHITELIST = ['http://localhost:5000',
             'http://localhost:3449',
             'http://localhost:9500']

cors = CORS(
    app,
    resources={
        r"/api/*": {
            "origins": WHITELIST,
            "expose_headers": "*",
            "allow_headers": "*",
            "supports_credentials": True
        }
    }
)


# @app.after_request
# def add_cors_headers(response):
#     r = request.headers['Origin']
#     if r in WHITELIST:
#         response.headers.add(
#             'Access-Control-Allow-Origin', r)
#         response.headers.add(
#             'Access-Control-Allow-Credentials', 'true')
#         response.headers.add(
#             'Access-Control-Allow-Headers', 'Content-Type')
#         response.headers.add(
#             'Access-Control-Allow-Headers', 'Cache-Control')
#         response.headers.add(
#             'Access-Control-Allow-Headers', 'X-Requested-With')
#         response.headers.add(
#             'Access-Control-Allow-Headers', 'Authorization')
#         response.headers.add(
#             'Access-Control-Allow-Methods', 'GET, POST, OPTIONS, PUT, DELETE')
#     return response


@app.route('/api/v1/time', methods=['GET'])
def get_time():
    return jsonify({
        'time': datetime.now().strftime("%H:%M:%S")})


app.run()
