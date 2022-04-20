# Camera Service

This is the `Camera Service` for the `CubeSat` project. The service is currently a `Skeleton`, low level operations need to be implemented.

This service listens on: http://0.0.0.0:8123/graphql

For testing purpose use: http://0.0.0.0:8123/graphiql

## Development

```bash
cargo run -- -c config.toml &
```

## Queries

Capture query to fetch all Image details:

```bash
{
  subsystem {
    capture {
      bytes
      resolution {
        x
        y
      }
    }
  }
}
```

Response:

```bash
{
  "data": {
    "subsystem": {
      "capture": {
        "bytes": [
          1,
          1,
          1,
          1
        ],
        "resolution": {
          "x": 1920,
          "y": 1080
        }
      }
    }
  }
}
```

Capture query to fetch the Image byte stream:

```bash
{
  subsystem {
    capture {
      bytes
    }
  }
}
```

Response:

```bash
{
  "data": {
    "subsystem": {
      "capture": {
        "bytes": [
          1,
          1,
          1,
          1
        ]
      }
    }
  }
}
```

## Mutations

Start the camera:

```bash
mutation {
  startCamera
}
```

Response:

```bash
{
  "data": {
    "startCamera": true
  }
}
```

Stop the camera:

```bash
mutation {
  stopCamera
}
```

Response:

```bash
{
  "data": {
    "stopCamera": true
  }
}
```

Set the camera resolution:

```bash
mutation {
  setResolution(x: 200, y: 200)
}
```

Response:

```bash
{
  "data": {
    "setResolution": true
  }
}
```
