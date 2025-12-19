# Vids

> Small website to manage the videos recorded using a free security app on an old phone.
The Backend is set on a raspberry pie to manage the videos and a small frontend allows to browse, delete, archive or turn to gif the videos.  

## Todo / Could do
 - [ ] Flip the videos list from raw to backup
 - [ ] Style...
 - [ ] Filter videos
 - [ ] Docker Build and start not done.


## Tests and other stuff:
  - Test CORS:

```bash
curl -v --request OPTIONS 'http://localhost:9000' -H 'Origin: http://aetes.local' -H 'Access-Control-Request-Method: GET'
```




>>> to check
https://ssojet.com/parse-and-generate-formats/parse-and-generate-json-in-yew/




> [!NOTE]
> All done mostly to learn & play with Rust... (⌒‿⌒)/
> Haven't played with web framework yew in a while, need to refresh my memory


[![License: WTFPL](https://upload.wikimedia.org/wikipedia/commons/f/fa/WTFPL_badge.png)](/LICENSE.txt)
