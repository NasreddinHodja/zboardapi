#!/usr/bin/env bash

# GET all users
curl localhost:8000/api/users

# POST new user
curl -d '{"username": "zine", "password": "zine"}' \
    -H 'Content-Type: application/json' \
    localhost:8000/api/new-user

# GET get user by id
curl localhost:8000/api/user/1
# GET get user by name
curl localhost:8000/api/user/zine

# GET remove user by id
curl localhost:8000/api/remove-user/1
# GET remove user by name
curl localhost:8000/api/remove-user/zine


# GET all posts
curl localhost:8000/api/posts

# POST new post
curl -d '{"title": "pau no cu", "body": "de quem ta lendo"}' \
    -H 'Content-Type: application/json' \
    localhost:8000/api/new-post

# GET get post by id
curl localhost:8000/api/post/1

# GET remove post by id
curl localhost:8000/api/remove-post/1
