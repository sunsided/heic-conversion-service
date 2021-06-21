# HEIF (`.heic`) conversion API

`libheif.so` is required to be installed. If it is built [from source](https://github.com/strukturag/libheif) 
and default installed to `/usr/local/lib`, ensure that it is loaded before any system installation by specifying e.g.

```console
$ export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

Then run

```console
$ cargo run --bin heif-server
$ cargo run --bin heif-client
```

You can find example HEIF (`.heic`) images at [nokiatech.github.io/heif/examples.html](http://nokiatech.github.io/heif/examples.html).

## Viewing HEIF images

To view HEIF (`.heic`) images on Ubuntu, you man need a special codec:

- `sudo apt install heif-thumbailer` - This will allow seeing HEIF image thumbnails in Nautilus.
- `sudo apt install gpicview heif-gdk-pixbuf` - This will allow opening HEIF images
 using the `gpicview` image viewer.

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
$ grpcurl -plaintext -import-path ./proto -proto heif_api.proto -d '{"heif": "AA==", "quality": 95}' localhost:50051 heif_api.Convert/ConvertJpeg
```

[Bloom RPC]: https://github.com/uw-labs/bloomrpc
[gRPCurl]: https://github.com/fullstorydev/grpcurl
