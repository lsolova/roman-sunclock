# Roman Sunclock

Roman Sunclock Time is a local time based on the sun movement. Daytime and nighttime are splitted into 12 hours (sunrise is always 06:00, sunset is always 18:00). Idea came from [this BBC article][1].

Roman Sunclock calculates sunset and sunrise before and after current timestamp. Then it calculates minute length for 12 hours between sun changes (set and rise) and current time in _Roman Sunclock Time_. It differs from original [Roman timekeeping][2], because this provides always 60 minutes and minute length varies (less or more seconds). A minute could be more than 60 seconds in summer time and shorter in winter time depending on the sun movement.

This is an experiment

- to write _WASM (WebAssembly)_ module using _Rust_
- set up _PWA_ for offline usage
- _data caching_ to support places without GPS (it is required for first use)
- drawing on ~~_canvas_~~ _svg_ (adapted to mobile)

Some parts of the application were implemented on the easiest, maybe quick-and-dirty way. Main goal was testing new technologies.

## Resources

- [BBC article][1]
- Original icons from [iconfinder](https://www.iconfinder.com)
- [MDN: Geolocation coordinates](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)
- [Rust WASM book](https://rustwasm.github.io/docs/book/)
- [Wikipedia: Julian day](https://en.wikipedia.org/wiki/Julian_day)
- [Wikipedia: Sunrise equation](https://en.wikipedia.org/wiki/Sunrise_equation)
- [Wikipedia: Roman timekeeping][2]

[1]: https://www.bbc.com/future/article/20240328-the-ancient-roman-alternative-to-daylight-savings-time
[2]: https://en.m.wikipedia.org/wiki/Roman_timekeeping
