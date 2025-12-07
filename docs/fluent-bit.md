## Config
### INPUT
```bash
[INPUT]
    Name         tail
    Path         /var/log/nginx/access-json.log
    Tag          app1nginx.access
    Read_from_Head    On
    Db            /tmp/app1-nginx-json.db
    Mem_Buf_Limit 50MB
    Parser        nginxjson

```
### OUTOUT
```bash
[OUTPUT]
    Name         http
    Match        app1nginx.*
    Host         zege-domain.com
    Port         443
    URI          /api/v1/e/i/basic
    Format       json
    Retry_Limit  10
    tls          Off
    tls.verify   Off
    Json_date_key timestamp
```
### PARSER
```bash
[PARSER]
    Name   nginxjson
    Format json
    Time_Key time
    Time_Format %d/%b/%Y:%H:%M:%S %z

```
