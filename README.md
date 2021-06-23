# HEIF (`.heic`) conversion API

The [libheif decoder demo] can decode HEIF image in the browser.

`libheif.so` is required to be installed. If it is built [from source](https://github.com/strukturag/libheif) 
and default installed to `/usr/local/lib`, ensure that it is loaded before any system installation by specifying e.g.

```console
$ export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

To see log output, specify the `RUST_LOG` environment variable, e.g.

```console
$ export RUST_LOG=info,heif_server=debug
```

Then run

```console
$ cargo run --bin heif-server
$ cargo run --bin heif-client
```

Example output:

```
Jun 22 01:44:40.491  INFO heif_server: Starting HEIF conversion server
Jun 22 01:44:46.775  INFO heif_server::services::convert_service: Handling HEIC (2.25 MB) -> JPEG conversion request (q=65%)
Jun 22 01:44:47.867 DEBUG heif_server::services::convert_service: Decoding HEIC image took 1s 91ms 321us 761ns
Jun 22 01:44:47.867 DEBUG heif_server::converter::jpeg_encoder: Encoding 2866 x 3024 x 8bpp image (26 MB raw data)
Jun 22 01:44:47.944 DEBUG heif_server::services::convert_service: Encoding JPEG image took 77ms 714us 466ns
Jun 22 01:44:47.944  INFO heif_server::services::convert_service: Finished conversion, produced 1.78 MB JPEG (79.0%) in 1s 169ms 36us 227ns
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

[libheif decoder demo]: https://strukturag.github.io/libheif/
[Bloom RPC]: https://github.com/uw-labs/bloomrpc
[gRPCurl]: https://github.com/fullstorydev/grpcurl
