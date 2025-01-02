# Tiny Health Checker

A tool that can be used in OCI images to perform health checks under Docker.

Both Kubernetes and Docker have the capability to run health checks against your running containers, however, unlike Kubernetes Docker does not have a tool built-in to do this. It requires your application to have the functionality or for a tool to be present in the image.

Many folks use `curl` for this and it's not a bad tool, but it's overkill. In addition, many people would like to strip out tools like `curl` because they can be misused if your application is compromised.

The tiny health checker is a small binary that can only make HTTP requests to localhost. The port and path are configurable, and you can specify using the loopback adapter address, but that's it. This is sufficient for health checks, but should be difficult to misuse if your container is compromised.

## Usage

```
USAGE:
	thc

ENV:

	THC_PORT sets the port to which a connection will be made, default: 8080
	THC_PATH sets the path to which a connection will be made, default: `/`
	THC_CONN_TIMEOUT sets the connection timeout, default: 10
	THC_REQ_TIMEOUT sets the request timeout, default: 15
	THC_USE_LOOPBACK_ADDRESS 'true' to use 127.0.0.1 in place of 'localhost', default: `false`

	**NOTE** Host is not configurable and will always be localhost (or 127.0.0.1)
```

## Examples

Connects to `http://localhost:8080/`.

```
thc
```

Connects to `http://localhost:9090/`.

```
THC_PORT=9090 thc
```

Connects to `http://localhost:9090/foo`.

```
THC_PORT=9090 THC_PATH=/foo thc
```

* the leading slash (`/`) is optional
