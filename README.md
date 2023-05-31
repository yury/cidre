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
- Almost fully integrated async blocks
- Apple's C API interfaces

### Shortcuts

- length -> len
- count -> len
- source -> src
- destination -> dst
- buffer -> buf
- mutable -> mut
- operation -> op
- error -> err
- options -> opts
- command -> cmd
- throws -> _throws (not shortcut but indicator of exception)

