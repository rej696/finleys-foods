import os

from flask import Flask
from flask_cors import CORS


def create_app(test_config=None):
    """Create and configure the app"""
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_mapping(
        SECRET_KEY="dev",
        DATABASE=os.path.join(app.instance_path, "api.db"),
        CORS_HEADERS = 'Content-Type'
    )

    if test_config is None:
        app.config.from_pyfile("config.py", silent=True)
    else:
        app.config.from_mapping(test_config)

    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    from . import db
    db.init_app(app)

    from .time import bp as time_bp
    app.register_blueprint(time_bp)

    CORS(
        app,
        resources={r"/api/*": {
            "origins": [
                'http://localhost:5000',
                'http://localhost:3449',
                'http://localhost:9500',
            ],
            "expose_headers": "*",
            "allow_headers": "*",
            "supports_credentials": True
        }}
    )

    return app
