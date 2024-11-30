#!/bin/bash

cd /workspace/marksurvey

source ./.env

echo $GIT_USER_NAME
echo $GIT_USER_EMAIL
git config --global user.name $GIT_USER_NAME
git config --global user.email $GIT_USER_EMAIL
git config --list | egrep "name|email" 
