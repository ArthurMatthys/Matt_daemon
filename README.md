# Running the project

1. build the base container
```shell
docker build . -t test-matt_daemon
docker run --label matt -ti test-matt_daemon bash
```

2. run this command as many times:
```shell
docker exec -ti $(docker ps --filter label=matt -q)  bash
```
