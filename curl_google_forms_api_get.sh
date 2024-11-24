#!/bin/bash

source ./.env

ACCESS_TOKEN=$(curl -s -X POST -d "code=${AUTHORIZATION_CODE}&client_id=${CLIENT_ID}&client_secret=${CLIENT_SECRET}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&grant_type=authorization_code" https://oauth2.googleapis.com/token | jq ".access_token")

echo "access_token:"
echo $ACCESS_TOKEN 

curl -H "Authorization: Bearer ${ACCESS_TOKEN}" "https://forms.googleapis.com/v1/forms/${FORM_ID}"
