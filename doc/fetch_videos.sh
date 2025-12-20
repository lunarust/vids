#!/bin/bash
#===============================================================================
#         FILE: fetch_videos.sh
#       AUTHOR: Celine
# ORGANIZATION: ---
#      VERSION: 0.0.1
#      CREATED: 12-12-2025
#         TODO: Build a UI to see the video?
#               Is it possible to extract a frame with something that changed from the rest?
#          VAR: Variables are fetched from a .videopass file with the following entries:
#               USERNAME=MyGreatUser
#               PASSWORD=MyAwesomePassword
#               IP=192.168.0.1:8080
#===============================================================================

if [ $# -ne 2 ]
then
  echo "
  -------------------
  An error occurred,
  Not enough or incorrect arguments supplied
  Ex: $0 'true|false' 'IP'
  -------------------"
  exit 1
fi

source .videopass
remove=$1
IP=$2
LAST=${IP##*.}
LTFILES=`curl -s http://${USERNAME}:${PASSWORD}@${IP}:8080/list_videos | jq -Mr '.[] | "\(.path)/\(.name)"'`

for entry in $LTFILES; do
  DIR="$(dirname "${entry}")"
  FILE="$(basename "${entry}")"
  FILENAME="${FILE%.*}"
  if [ -e /opt/vids/raw/${LAST}/$FILE ]; then
    if [ ${remove} = "true" ]; then
      echo "${entry} for ${IP} > ${LAST} exists, I can remove it from the phone"
      #echo " curl --location --request POST http://${IP}/remove${entry} --header 'Content-Type: application/json' --digest -u ${USERNAME}:${PASSWORD} "
      curl --location --request POST "http://${USERNAME}:${PASSWORD}@${IP}:8080/remove${entry}" \
           --header 'Content-Type: application/json' \
           --header "uri: /remove${entry}"
    fi
  else
    echo "${entry} for ${IP} > ${LAST} doesnt exist, must dowload"
    wget -P /opt/vids/raw/${LAST}/ "http://${USERNAME}:${PASSWORD}@${IP}:8080/v${entry}"
  fi
done

echo "Completed, be polite and say bye, bye now ヾ(^_^)"
