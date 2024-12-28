# Cidre (French Cidr) - Rusty Apple API's

## Goals

- Performance 
- Zero cost objc interop
- No legacy platforms support
- ML friendly
- Rusty API (ObjC and C) 
- API Availability checks
- Buttle tested in own apps. (StreamChamp.app for recording, Yoml to be released)

### Performance 

- Developer can finish critical path without deallocations.
- Zero cost selectors calls (XCode 14.3) [WWDC video 3:10](https://developer.apple.com/videos/play/wwdc2022/110363/)
- Integrated async blocks
- Zero cost Apple's C API interfaces
- Static `cf::String` and `ns::String` refs (`ns::str!(c"hello")`) 

### Platforms

- [x] macOS
- [x] iOS/iPadOS
- [x] tvOS
- [ ] watchOS
- [ ] visionOS

### iOS devices runner

Run tests on your iPhone or iPad.

1. Run `cargo install --path ./cargo-box` to install cargo box plugin
2. Run `cargo box teams` to find out your DEVELOMPENT_TEAM id
3. Run `cargo box devices` to find out your DEVICE_ID
4. Create `.box` file with contents:
```
BOX_ORG_ID = unique org id, for instance org.cidre (it may be reserved already)
DEVELOPMENT_TEAM = team id from step 2
DEVICE_ID = device id from step 3
```
5. Run `cargo t --target aarch64-apple-ios` (make sure you have connected and unlocked device)
6. Run `cargo r --target aarch64-apple-ios --example device-formats`

### Versioning (API Availability)

Deployment targets are controlled via features `macos_x_x`, `ios_x_x`, `tvos_x_x`, `watchos_x_x`, `visionos_x_x`.
If selector is not defined in deployment target it becomes unsafe. So developer
should check if object responses to that selector before call it.

Default features: `macos_15_0`, `ios_18_0`, `tvos_18_0`, `maccatalyst_18_0`, `watchos_11_0`, `visionos_2_0`;

### Shortcuts

- address -> addr
- argument -> arg
- attachment -> attach
- attribute -> attr
- attributted -> attr
- buffer -> buf
- command -> cmd
- count -> len
- descriptor -> desc
- description -> desc
- destination -> dst
- error -> err
- extension -> ext
- lavel -> lvl
- language -> lang
- length -> len
- mutable -> mut
- operation -> op
- options -> opts
- pointer -> ptr
- source -> src
- surface - surf
- throws -> _throws (not shortcut but indicator of possible exception)

## About

This is personal research project. With this project I learn Apple SDK's and rust.
