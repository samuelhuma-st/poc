# HTTP Hello World

This is a simple Rust Wasm example that responds with a "Hello World" message for each request.

## Prerequisites

- `cargo` 1.75
- [`wash`](https://wasmcloud.com/docs/installation) 0.27.0
- `wasmtime` >=19.0.0 (if running with wasmtime)

## Building

```bash
wash build
```

## Running with wasmtime

You must have wasmtime >=19.0.0 for this to work. Make sure to follow the build step above first.

```bash
wasmtime serve -Scommon ./build/http_hello_world_s.wasm
```

## Running with wasmCloud

Ensuring you've built your component with `wash build`, you can launch wasmCloud and deploy the full hello world application with the following commands. Once the application reports as **Deployed** in the application list, you can use `curl` to send a request to the running HTTP server.

```shell
wash up -d
wash app deploy ./wadm.yaml
wash app list
curl http://127.0.0.1:8080
```

## Adding Capabilities

To learn how to extend this example with additional capabilities, see the [Adding Capabilities](https://wasmcloud.com/docs/tour/adding-capabilities?lang=rust) section of the wasmCloud documentation.


## Template workflow data
```
{
  "name": "Poc",
  "nodes": [
    {
      "id": "3",
      "name": "Display",
      "node_type": "print",
      "parameters": {
        "value": "{{$(start).json.description}}. We are {{$(Addition order).json.result}} order today. I use add node to get order because {{$(Addition order).json.description}}"
      }
    },
    {
      "id": "2",
      "name": "Addition order",
      "node_type": "add",
      "parameters": {
        "value": [
          1,
          5,
          4
        ]
      },
      "next_node": "3"
    },
    {
      "id": "5",
      "name": "start",
      "node_type": "trigger",
      "parameters": {},
      "next_node": "2"
    }
  ],
  "connections": [
    {
      "from": "1",
      "to": "2"
    },
    {
      "from": "2",
      "to": "3"
    }
  ],
  "metadata": {
    "description": "A poc workflow"
  }
}
```