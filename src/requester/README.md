# Endpoint Requester

This provides an `EndpointRequester` struct for making JSON RPC requests.

## Structs

| Struct Name | Fields |
|-------------|--------|
| Options | headers: HeaderMap, query_params: HashMap<String, String> |
| EndpointRequester | client: reqwest::blocking::Client, uri: Url, base: String |

## Methods

| Method Name | Struct | Parameters | Return Type |
|-------------|-----------|------------|-------------|
| new | Options | None | Options |
| with_header | Options | key: &str, val: &str | Options |
| with_query_params | Options | key: &str, val: &str | Options |
| new | EndpointRequester | uri: Url, base: String | EndpointRequester |
| send_request | EndpointRequester | method: &str, params: &T, reply: &mut R, options: Options | Result<(), Box Error> |

## Description

### Options

`Options` is a struct that represents the options for a request. It has the following fields:

- `headers`: The headers of the request.
- `query_params`: The query parameters of the request.

It also has methods for creating a new `Options` instance and for adding headers and query parameters.

### EndpointRequester

`EndpointRequester` is a struct that represents an endpoint requester. It has the following fields:

- `client`: The HTTP client.
- `uri`: The URI of the endpoint.
- `base`: The base of the method.