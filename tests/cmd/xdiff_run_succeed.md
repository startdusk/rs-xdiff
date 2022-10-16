# xdiff run succeed

```trycmd
$ xdiff run -p todo -c  ./fixtures/test.yml -e a=10 -e @b=2 -e %c=3 -e m=10
1   1    | HTTP/2.0 200 OK
2   2    | content-type: "application/json; charset=utf-8"
3        |-content-length: "83"
    3    |+content-length: "99"
4   4    | x-powered-by: "Express"
5   5    | x-ratelimit-limit: "1000"
6   6    | vary: "Origin, Accept-Encoding"
--------------------------------------------------------------------------------
9   9    | pragma: "no-cache"
10  10   | expires: "-1"
11  11   | x-content-type-options: "nosniff"
12       |-etag: "[..]"
    12   |+etag: "[..]"
13  13   | via: "1.1 vegur"
14  14   | accept-ranges: "bytes"
15  15   | server: "cloudflare"
--------------------------------------------------------------------------------
17  17   | 
18  18   | {
19  19   |   "completed": false,
20       |-  "title": "delectus aut autem",
    20   |+  "title": "quis ut nam facilis et officia qui",
21  21   |   "userId": 1
22  22   | }
```