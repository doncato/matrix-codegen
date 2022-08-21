# matrix-codegen
A simple command line tool to generate QR codes and DataMatrix Codes

Contributions are welcome! You can also contribute by adding other 1D or 2D code
standards!

# Installation
Navigate to the projects release page, choose a release and download the appropriate
binary file. Or compile it from source if you are a rustacean

# Usage:
**Note:** If in doubt, use the `--help` option

## QR Code
To generate a QR Code which says "You scanned me" and save it as "qr.png":
```
$ ./matrix-codegen "You scanned me" qr.png -q
```
The same example with defined image size and error correction scheme (for error correction High):
```
$ ./matrix-codegen "You scanned me" qr.png -q -s 512 -e h
```
The following error correction schemes can be used:
`l`,`m`,`q`,`h`
for low, medium, quartile and high error correction.

## DataMatrix Code
To generate a DataMatrix Code which says "You scanned me" and save it as "datamatrix.png":
```
$ ./matrix-codegen "You scanned me" datamatrix.png -d
```
The same example with defined size:
**Note:** with DataMatrix the size defines the pixels one black square represents, __not__ the resulting image size!
```
$ ./matrix-codegen "You scanned me" datamatrix.png -d -s 10
```
