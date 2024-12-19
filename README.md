# portfolio

[ethan.armstronglabs.net](https://ethan.armstronglabs.net)

## Workflow

```cross build --release --target x86_64-unknown-linux-musl```

```docker build . -t explosion33/portfolio:latest```

```docker push explosion33/portfolio```

then to run on server

```docker pull explosion33/portfolio```

```docker run -p external:internal --name portfolio explosion33/portfolio```

currently: `internal = 80`
