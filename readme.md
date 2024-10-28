# Roman Sun Clock

This is an experiment to write WASM (WebAssembly) module using _Rust_. Idea came from [this BBC article](https://www.bbc.com/future/article/20240328-the-ancient-roman-alternative-to-daylight-savings-time).

Roman Sun Clock calculates sunset and sunrise of yesterday, today and tomorrow. Based on the current timestamp it calculates minute length for 12 hours between sun changes (set and rise) and current time in _Roman Sun Time_. A minute is longer than 60 seconds in summer time and shorter in winter time depending on the sun movement.

## Resources

- [Icons from iconfinder](https://www.iconfinder.com)
- [MDN: Geolocation coordinates](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)
- [Rust WASM book](https://rustwasm.github.io/docs/book/)
- [Wikipedia: Julian day](https://en.wikipedia.org/wiki/Julian_day)
- [Wikipedia: Sunrise equation](https://en.wikipedia.org/wiki/Sunrise_equation)
