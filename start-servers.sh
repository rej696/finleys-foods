#! /bin/bash
{
    cd backend;
    source venv/bin/activate;
    echo "Starting Flask Server";
    python wsgi.py;
} > flask-log.txt 2>&1 &
{
    cd frontend-clj;
    echo "Starting Figwheel Server";
    clj -M:dev;
} > figwheel-log.txt 2>&1 &
