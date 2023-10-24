# Cidre (French Cidr) - Rusty Apple API's

## Goals

- Performance 
- Zero cost objc interop
- No legacy platforms support
- ML friendly
- Rusty API (ObjC and C) 

### Performance 

- Developer can finish critical path without deallocations.
- Zero cost selectors calls (XCode 14.3) [WWDC video 3:10](https://developer.apple.com/videos/play/wwdc2022/110363/)
- Integrated async blocks
- Zero cost Apple's C API interfaces

### Platforms

- [x] MacOS
- [x] iOS/iPadOS
- [ ] tvOS
- [ ] watchOS

### Shortcuts

- argument -> arg
- buffer -> buf
- command -> cmd
- count -> len
- descriptor -> desc
- destination -> dst
- error -> err
- length -> len
- mutable -> mut
- operation -> op
- options -> opts
- pointer -> ptr
- source -> src
- surface - surf
- throws -> _throws (not shortcut but indicator of exception)

## About

This is personal research project. With this project I learn Apple SDK's and rust.