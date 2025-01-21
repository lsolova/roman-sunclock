# Roman Sunclock Time

Roman Sunclock Time is a local time based on the sun movement. Daytime and nighttime are splitted into 12 hours (sunrise is always 06:00, sunset is always 18:00). Idea came from [this BBC article][1].

Roman Sunclock calculates sunset and sunrise before and after current timestamp. Then it calculates minute length for 12 hours between sun changes (set and rise) and current time in _Roman Sunclock Time_. It differs from original [Roman timekeeping][2], because this provides always 60 minutes and minute length varies (less or more seconds). A minute could be more than 60 seconds in summer time and shorter in winter time depending on the sun movement. Daytime starts a 6:00 RSCT and finishes at 18:00 RSCT.

This is an experiment

- to write _WASM (WebAssembly)_ module using _Rust_
- set up _PWA_ for offline usage and installable app
- _data caching_ to support places without GPS (it is required for first use)
- drawing on ~~_canvas_~~ _svg_ (adapted to mobile)

Some parts of the application were implemented on the easiest, maybe quick-and-dirty way. Main goal was testing new technologies.

Application uses free tier of reverse geocoding API by [Geoapify](https://www.geoapify.com/).

## Usage

Open [Roman Sunclock Time][3] app and allow using location. It can be installed as an app.

For a certain point on earth, it is possible to provide GPS coordinates in the URL. It goes into the hash part, with a pipe (`|`) separator. Examples:

- Location only: <https://rsct.solova.com/#36.6955794844035|-4.4513623935250655|0>
- Location and time: <https://rsct.solova.com/#68.2992471|22.2632669|0|2024-11-15T16:55:00>

> Note: If location is defined such a way, local time might be inaccurate, due to timezone differences.

## Under the hood

This application uses astronomical calculations to calculate sunrise, sunset (Sun's position relative to observer's horizon), moonrise, moonset (Moon's position relative to observer's horizon) and illuminated fraction of Moon (illuminated area of the Moon visible from Earth).

Calculations are based on [Julian Day Number (JDN)][4].

## Resources

- Astronomical Algorithms (second edition) by Jean Meeus
- [Astronomical Calculations: Solar Coordinates by James Still](https://squarewidget.com/solar-coordinates/)
- [BBC article][1]
- [Celestial programming](https://celestialprogramming.com/) site by Greg Miller
- [MDN: Geolocation coordinates](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)
- Original icons from [iconfinder](https://www.iconfinder.com)
- [Rust WASM book](https://rustwasm.github.io/docs/book/)
- [Wikipedia: Elongation](https://en.wikipedia.org/wiki/Elongation_(astronomy))
- [Wikipedia: Julian day][4]
- [Wikipedia: Position of the Sun](https://en.wikipedia.org/wiki/Position_of_the_Sun)
- [Wikipedia: Roman timekeeping][2]
- [Wikipedia: Sunrise equation](https://en.wikipedia.org/wiki/Sunrise_equation)

[1]: https://www.bbc.com/future/article/20240328-the-ancient-roman-alternative-to-daylight-savings-time
[2]: https://en.m.wikipedia.org/wiki/Roman_timekeeping
[3]: https://rsct.solova.com
[4]: https://en.wikipedia.org/wiki/Julian_day
