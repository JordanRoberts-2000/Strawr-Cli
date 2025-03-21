grab command works only with keychain, asking for password on set and get is possible but current encryption is disabled without keychain.

[image]
Progressive jpeg / Interlaced png detection and conversion.

- Adds extra overhead to the image service
- Decoding progressive files will remove the progressive part, requiring special attention,
- Not supported by the image crate
- Ultimatly due to improved global internet speed and better compression algorithms and progressive files having a larger file size,
  decided against supporting progressive images at this time

Avif

- not supported by the image crate
- Hardware decoders and encoders not on enough devices to see consistent performance benifits compared to webp
- ravif crate used for encoding avif files, causing issues with rust analyzer
