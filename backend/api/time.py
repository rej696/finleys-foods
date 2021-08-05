from datetime import datetime

from flask import Blueprint, jsonify
from api.db import get_db


bp = Blueprint('time', __name__, url_prefix='/api/v1/time')


@bp.route('/users', methods=['GET'])
def get_users():
    db = get_db()
    users = []
    for row in db.execute('select username from users').fetchall():
        users.append(row['username'])
    return jsonify({'users': users})


@bp.route('/', methods=['GET'])
def get_time():
    return jsonify({
        'time': datetime.now().strftime('%H:%M:%S')})
