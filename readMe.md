### TCP Socket Server


### Dev
```bash
- cp .env.example .env
- diesel setup
- diesel migration run
- cargo run
```
### Client Connect
```bash
nc 127.0.0.1 4567
{"tag":"Login","data":"{\"username\":\"admin\",\"password\":\"123456\"}"}
{"tag":"Register","data":"{\"username\":\"admin\",\"password\":\"123456\"}"}
```
