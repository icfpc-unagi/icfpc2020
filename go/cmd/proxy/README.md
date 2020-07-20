## How to build

```
docker build -t imos/icfpc2020:proxy \
    --build-arg ENDPOINT='https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=*******' \
    -f proxy.Dockerfile .
```

## How to use

Bind your port (e.g., 12345) for the proxy server.

```bash
docker run -p 12345:8080 --rm -it imos/icfpc2020:proxy run
```
