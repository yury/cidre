# Cidre (French Cidr) - Rusty Apple API's

## Goals

- Performance 
- Zero cost objc interop
- No legacy platforms support
- ML friendly
- Rusty API (ObjC and C) 
- API Availability checks

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
- [x] watchOS
- [ ] visionOS

### Versioning (API Availability)

Deployment targets are controlled via features `macos_x_x`, `ios_x_x`, `tvos_x_x`, `watchos_x_x`, `visionos_x_x`.
If selector is not defined in deployment target it becomes unsafe. So developer
should check if object responses to that selector before call it.

Default features: `macos_14_0`, `ios_17_0`, `tvos_17_0`, `maccatalyst_17_0`, `watchos_10_0`, `visionos_1_0`;

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
