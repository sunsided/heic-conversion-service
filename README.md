# HEIF conversion API

```console
$ cargo run --bin heif-server
$ cargo run --bin heif-client
```

## Local configuration

To change the service defaults, create a `.env` file with the following properties,
then adjust them to your needs:

```env
GRPC_SERVER_ADDRESS=127.0.0.1:50051
GRPC_SERVER_SCHEME=http
```

## Example use

If you have a gRPC GUI client such as [Bloom RPC] you should be able to send requests to the server and get back results!
If you use [gRPCurl] then you can simply try sending requests like this:

```console
$ grpcurl -plaintext -import-path ./proto -proto convert.proto -d '{"heif": "AA==", "quality": 95}' localhost:50051 heif_api.Convert/ConvertJpeg
```

[Bloom RPC]: https://github.com/uw-labs/bloomrpc
[gRPCurl]: https://github.com/fullstorydev/grpcurl
