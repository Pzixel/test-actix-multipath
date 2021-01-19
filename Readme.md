Run two querties:

```bash
curl "http://localhost:8800/foo" --verbose
```
```bash
curl -X PUT "http://localhost:8800/foo" -d "{}" --verbose
```

See one o them working properly and another one failing with 405:

```bash
pzixe@ZPC MINGW64
$ curl "http://localhost:8800/foo" --verbose
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0*   Trying ::1:8800...
*   Trying 127.0.0.1:8800...
* Connected to localhost (127.0.0.1) port 8800 (#0)
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0> GET /foo HTTP/1.1
> Host: localhost:8800
> User-Agent: curl/7.69.1
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 0
< date: Tue, 19 Jan 2021 16:35:59 GMT
<
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
* Connection #0 to host localhost left intact

pzixe@ZPC MINGW64
$ curl -X PUT "http://localhost:8800/foo" -d "{}" --verbose
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0*   Trying ::1:8800...
*   Trying 127.0.0.1:8800...
* Connected to localhost (127.0.0.1) port 8800 (#0)
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0> PUT /foo HTTP/1.1
> Host: localhost:8800
> User-Agent: curl/7.69.1
> Accept: */*
> Content-Length: 2
> Content-Type: application/x-www-form-urlencoded
>
} [2 bytes data]
* upload completely sent off: 2 out of 2 bytes
* Mark bundle as not supporting multiuse
< HTTP/1.1 405 Method Not Allowed
< content-length: 0
< date: Tue, 19 Jan 2021 16:36:04 GMT
<
100     2    0     0  100     2      0      9 --:--:-- --:--:-- --:--:--     9
* Connection #0 to host localhost left intact


```