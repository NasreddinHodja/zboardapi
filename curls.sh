#!/usr/bin/env bash

# GET all users
curl localhost:8000/api/users

# POST new user
curl -d '{"username": "zine", "password": "zine"}' \
    -H 'Content-Type: application/json' \
    localhost:8000/api/new-user

# POST get user by username
curl -d '{"username": "zine"}' \
    -H 'Content-Type: application/json' \
    localhost:8000/api/get-user

# POST remove user by username
curl -d '{"username": "prates"}' \
    -H 'Content-Type: application/json' \
    localhost:8000/api/remove-user
