import * as wasm from "roman-clock";

function getLocation() {
  return new Promise((resolve, reject) => {
    const options = {
      enableHighAccuracy: true,
      timeout: 5000,
      maximumAge: 0,
    };

    function success(pos) {
      const crd = pos.coords;

      console.log("Your current position is:");
      console.log(`Latitude : ${crd.latitude}`);
      console.log(`Longitude: ${crd.longitude}`);
      console.log(`Elevation: ${crd.altitude}`);
      console.log(`More or less ${crd.accuracy} meters.`);
      resolve({
        lat: crd.latitude,
        lon: crd.longitude,
        alt: crd.altitude || 0,
      });
    }

    function error(err) {
      reject(`ERROR(${err.code}): ${err.message}`);
    }

    navigator.geolocation.getCurrentPosition(success, error, options);
  });
}

getLocation().then((locationDetails) => {
  const romanSunTime = wasm.roman_sun_time(
    BigInt(Date.now()),
    locationDetails.lat,
    locationDetails.lon,
    locationDetails.alt
  );
  const dayOrNightElement = document.getElementById("dayOrNight");
  dayOrNightElement.classList.add(
    `RomanClock__DayOrNight--${romanSunTime.is_day ? "day" : "night"}`
  );
  const nowTimeElement = document.getElementById("nowTime");
  nowTimeElement.appendChild(
    document.createTextNode(
      `${Math.floor(romanSunTime.total_minutes / 60)
        .toString()
        .padStart(2, "0")}:${(romanSunTime.total_minutes % 60)
        .toString()
        .padStart(2, "0")}`
    )
  );
  const minuteLengthElement = document.getElementById("minuteLength");
  minuteLengthElement.appendChild(
    document.createTextNode(
      `${new Intl.NumberFormat("en", { maximumFractionDigits: 2 }).format(
        romanSunTime.minute_length
      )} seconds`
    )
  );

  const lastSunChange = romanSunTime.last_sun_change.toString();
  const nextSunChange = romanSunTime.next_sun_change.toString();
  console.log(lastSunChange, new Date(Number(lastSunChange)));
  console.log(nextSunChange, new Date(Number(nextSunChange)));
  console.log(romanSunTime.hours, romanSunTime.minutes);
});
