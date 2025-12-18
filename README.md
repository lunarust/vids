# Rust web template


## Todo / Could do

  - Test CORS:

```bash
curl -v --request OPTIONS 'http://bors.greece.local:9000' -H 'Origin: http://aetes.greece.local' -H 'Access-Control-Request-Method: GET'
```

  - Test backend

```  
curl --location "http://localhost:9000/soundout" \
  --header "Content-Type: application/json" \
  --data '[{"id":1,"name":"modet_2025-12-12_18-35.mp4","path":"/opt/vids/backup/modet_2025-12-12_18-35.mp4","what":"video","url":"http://bors.greece.local/vids/backup/modet_2025-12-12_18-35.mp4"}]'

```
Works !!!
curl --location 'http://localhost:9000/soundout' \
--header 'Content-Type: application/json' \
--data '"{name:modet_2025-12-12_18-35.mp4,path:/opt/vids/backup/modet_2025-12-12_18-35.mp4}"'
Works !!!




>>> to check
https://ssojet.com/parse-and-generate-formats/parse-and-generate-json-in-yew/


{"id":1,"name":"modet_2025-12-12_18-35.mp4","path":"/opt/vids/backup/modet_2025-12-12_18-35.mp4","what":"video","url":"http://bors.greece.local/vids/backup/modet_2025-12-12_18-35.mp4"}

 "{"VideoRequest":{"path":"plop","name":"dsa"}}"
"path":"plop","name":"dsa"
curl -X POST http://localhost:9000/soundout \
   -H 'Content-Type: application/json' \
   -d "{name:my_login,path:my_password}"

"{path:dedefe,adsa:dsada}"

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name": "modet_2025-12-12_18-35.mp4","path": "/opt/vids/backup/modet_2025-12-12_18-35.mp4"}' \
  http://localhost:9000/soundout


curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"id":1,"name":"modet_2025-12-12_18-35.mp4","path":"/opt/vids/backup/modet_2025-12-12_18-35.mp4","what":"video","url":"http://bors.greece.local/vids/backup/modet_2025-12-12_18-35.mp4"}' \
  http://localhost:9000/soundout


curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"xyz","password":"xyz"}' \
  http://localhost:3000/api/login

> [!NOTE]
> All done mostly to learn & play with Rust... (⌒‿⌒)/
> Haven't played with web framework yew in a while, need to refresh my memory


[![License: WTFPL]
