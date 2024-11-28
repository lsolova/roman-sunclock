import * as wasm from "roman-sunclock";

function asNumber(bigint) {
  return Number(bigint.toString());
}

function formatClockTime(hours, minutes) {
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}`;
}

function generateSvg(
  nowTime,
  todayStartEpoch,
  todaySunriseEpoch,
  todaySunsetEpoch
) {
  const CANVAS_X_CENTER = 125;
  const CANVAS_Y_CENTER = 125;
  const DAYLENGTH = 86400000;
  const FULL_CIRCLE = Math.PI * 2;
  const NINETY_DEGREE_IN_RAD = Math.PI / 2;

  function calculatePoint(w, radius) {
    return [
      Math.cos(w - NINETY_DEGREE_IN_RAD) * radius + CANVAS_X_CENTER,
      Math.sin(w - NINETY_DEGREE_IN_RAD) * radius + CANVAS_Y_CENTER,
    ];
  }

  function drawRomanHourLines(initialW, step) {
    const content = [];
    for (let i = 1; i < 12; i++) {
      const w = initialW + i * step;
      const length = i % 3 === 0 ? 107 : 113;
      const [x1, y1] = calculatePoint(w, 119);
      const [x2, y2] = calculatePoint(w, length);
      content.push(`<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" />`);
    }
    return content;
  }

  const timezoneOffset = new Date().getTimezoneOffset() * 60 * 1000;
  const nowDiff = nowTime - todayStartEpoch - timezoneOffset;
  const nowW = (nowDiff / DAYLENGTH) * FULL_CIRCLE;
  const sunriseEpochDiff = todaySunriseEpoch - todayStartEpoch - timezoneOffset;
  const sunriseW = (sunriseEpochDiff / DAYLENGTH) * FULL_CIRCLE;
  const sunsetEpochDiff = todaySunsetEpoch - todayStartEpoch - timezoneOffset;
  const sunsetW = (sunsetEpochDiff / DAYLENGTH) * FULL_CIRCLE;

  const [nowPointX, nowPointY] = calculatePoint(nowW, 102);
  const [sunriseX, sunriseY] = calculatePoint(sunriseW, 105);
  const [sunsetX, sunsetY] = calculatePoint(sunsetW, 105);

  const isDayLonger = Math.PI < sunsetW - sunriseW;

  const svgContent = [];
  svgContent.push('<svg viewBox="0 0 250 250" fill="transparent">');
  svgContent.push(
    `<path d="M ${sunsetX} ${sunsetY} A 105 105 0 ${
      isDayLonger ? "0 1" : "1 1"
    } ${sunriseX} ${sunriseY}" stroke="var(--night-color)" stroke-width="30" />`
  );
  svgContent.push(
    `<path d="M ${sunriseX} ${sunriseY} A 105 105 0 ${
      isDayLonger ? "1 1" : "0 1"
    } ${sunsetX} ${sunsetY}" stroke="var(--day-color)" stroke-width="30" />`
  );
  svgContent.push('<g stroke="var(--main-color)">');
  svgContent.push(
    `<circle cx=\"${CANVAS_X_CENTER}\" cy=\"${CANVAS_Y_CENTER}\" r=\"90\" />`
  );
  svgContent.push(
    `<circle cx=\"${CANVAS_X_CENTER}\" cy=\"${CANVAS_Y_CENTER}\" r=\"120\" />`
  );
  for (let i = 0; i < 24; i++) {
    const w = (i * Math.PI) / 12;
    const length = i % 3 === 0 ? 80 : 85;
    const [x1, y1] = calculatePoint(w, 91);
    const [x2, y2] = calculatePoint(w, length);
    svgContent.push(`<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" />`);
  }
  svgContent.push('<g stroke="var(--night-color)">');
  svgContent.push(...drawRomanHourLines(sunriseW, (sunsetW - sunriseW) / 12));
  svgContent.push("</g>");
  svgContent.push('<g stroke="var(--day-color)">');
  svgContent.push(
    ...drawRomanHourLines(sunsetW, (FULL_CIRCLE - (sunsetW - sunriseW)) / 12)
  );
  svgContent.push("</g>");
  svgContent.push("</g>");
  svgContent.push(
    `<circle cx=\"${nowPointX}\" cy=\"${nowPointY}\" r=\"4\" fill=\"red\" />`
  );
  // svgContent.push(
  //   `<circle cx=\"${sunsetX}\" cy=\"${sunsetY}\" r=\"4\" fill=\"purple\" />`
  // );
  // svgContent.push(
  //   `<circle cx=\"${sunriseX}\" cy=\"${sunriseY}\" r=\"4\" fill=\"orange\" />`
  // );
  svgContent.push("</svg>");
  return svgContent.join("");
}

function saveLocationDetails(locationDetails) {
  window.localStorage.setItem(
    "rsctLocationDetails",
    JSON.stringify({
      ...locationDetails,
      expiry: Date.now() + 86400000,
    })
  );
}

function loadLocationDetails() {
  const locationDetailsString = window.localStorage.getItem(
    "rsctLocationDetails"
  );
  if (locationDetailsString) {
    const locationDetails = JSON.parse(loadLocationDetailsString);
    if (Date.now() < loadLocationDetails.expiry) {
      return locationDetails;
    }
  }
  return null;
}

function saveLocationName(lat, lon, locationName) {
  window.localStorage.setItem(
    "rsctLocationName",
    JSON.stringify({
      lat,
      lon,
      locationName,
    })
  );
}

function loadLocationName(lat, lon) {
  const locationNameString = window.localStorage.getItem("rsctLocationName");
  if (locationNameString) {
    const locationName = JSON.parse(locationNameString);
    const knownLat = locationName.lat;
    const knownLon = locationName.lon;
    if (
      lat + 0.5 > knownLat &&
      lat - 0.5 < knownLat &&
      lon + 0.5 > knownLon &&
      lon - 0.5 < knownLon
    ) {
      return locationName.locationName;
    }
  }
  return null;
}

function setNotification(notification) {
  const notificationE = document.querySelector(".RomanSunclock__Notification");
  notificationE.innerHTML = notification;
}

function getLocation() {
  return new Promise((resolve, reject) => {
    const options = {
      enableHighAccuracy: true,
      timeout: 5000,
      maximumAge: 0,
    };

    function success(pos) {
      const crd = pos.coords;

      const locationDetails = {
        lat: crd.latitude,
        lon: crd.longitude,
        alt: crd.altitude || 0,
      };

      saveLocationDetails(locationDetails);
      resolve(locationDetails);
    }

    function error(err) {
      const locationDetails = loadLocationDetails();
      if (locationDetails) {
        resolve(loadLocationDetails);
        return;
      }
      reject(new Error(`ERROR(${err.code}): ${err.message}`));
    }

    navigator.geolocation.getCurrentPosition(success, error, options);
  });
}

function getLocationName(lat, lon) {
  return new Promise((resolve) => {
    const locationName = loadLocationName(lat, lon);
    if (locationName) {
      resolve(locationName);
      return;
    }
    fetch(
      `https://api.geoapify.com/v1/geocode/reverse?lat=${lat}&lon=${lon}&type=city&limit=1&format=json&apiKey=${reverseGeocodeApiKey}`
    )
      .then((response) => response.json())
      .then((result) => {
        if (Array.isArray(result.results) && result.results.length) {
          const reverseGeocode = result.results[0];
          const locationName = `${reverseGeocode.city}, ${reverseGeocode.country}`;
          saveLocationName(lat, lon, locationName);
          resolve(locationName);
        }
        resolve("");
      })
      .catch((e) => {
        console.log("Location could not be resolved", e);
        resolve("");
      });
  });
}

function updateCurrentRomanSunTime(locationDetails, locationName) {
  const nowDate = new Date();

  const romanSunTimeDetails = wasm.roman_sun_time(
    BigInt(nowDate.getTime()),
    locationDetails.lat,
    locationDetails.lon,
    locationDetails.alt
  );

  const todayStartEpoch = asNumber(romanSunTimeDetails.today_start);
  const todaySunriseEpoch = asNumber(romanSunTimeDetails.today_sunrise);
  const todaySunsetEpoch = asNumber(romanSunTimeDetails.today_sunset);
  const lastSunChangeEpoch = asNumber(romanSunTimeDetails.last_sun_change);
  const nextSunChangeEpoch = asNumber(romanSunTimeDetails.next_sun_change);

  const nowTime = formatClockTime(nowDate.getHours(), nowDate.getMinutes());
  const romanSunclockTime = formatClockTime(
    romanSunTimeDetails.hours,
    romanSunTimeDetails.minutes
  );
  const lastSunChangeTime = formatClockTime(
    new Date(lastSunChangeEpoch).getHours(),
    new Date(lastSunChangeEpoch).getMinutes()
  );
  const nextSunChangeTime = formatClockTime(
    new Date(nextSunChangeEpoch).getHours(),
    new Date(nextSunChangeEpoch).getMinutes()
  );

  const locationDetailsElement = document.getElementById("scDetails");
  const localNameElement = locationName ? `<span>${locationName}</span>` : "";
  locationDetailsElement.innerHTML = `
    ${localNameElement}<span>${new Intl.NumberFormat("en", {
    maximumFractionDigits: 2,
  }).format(romanSunTimeDetails.minute_length)} secs/min</span>
  <span>${
    romanSunTimeDetails.is_day ? "day" : "night"
  }time: ${lastSunChangeTime} - ${nextSunChangeTime}</span>
  `;

  const dayClockElement = document.getElementById("dayClockImage");
  dayClockElement.innerHTML = generateSvg(
    nowDate,
    todayStartEpoch,
    todaySunriseEpoch,
    todaySunsetEpoch
  );

  const dayClockHoursElement = document.getElementById("dayClockHours");
  dayClockHoursElement.innerHTML = `
    <div class="RomanSunclock__DayOrNight RomanSunclock__DayOrNight--${
      romanSunTimeDetails.is_day ? "day" : "night"
    }"></div>
    <div class="RomanSunclock__Clock__RomanTime">${romanSunclockTime}</div>
    <div class="RomanSunclock__Clock__LocalTime">${nowTime}</div>
  `;

  const rootE = document.getElementById("scRoot");
  rootE.classList.remove("RomanSunclock--loading");
  return romanSunTimeDetails.minute_length;
}

navigator.permissions
  .query({ name: "geolocation" })
  .then(({ state }) => {
    if (state === "granted") {
      setNotification("Requesting your location. Please, wait.");
    } else {
      setNotification(
        "Please, allow location services to get your local Roman Sunclock Time."
      );
    }
    return getLocation();
  })
  .then((locationDetails) => {
    const { lat, lon } = locationDetails;
    const locationNamePromise = getLocationName(lat, lon);
    return locationNamePromise.then((locationName) => ({
      locationDetails,
      locationName,
    }));
  })
  .then(({ locationDetails, locationName }) => {
    const intervalLength = updateCurrentRomanSunTime(
      locationDetails,
      locationName
    );
    document.addEventListener("visibilitychange", () => {
      if (!document.hidden) {
        updateCurrentRomanSunTime(locationDetails, locationName);
      }
    });
    setInterval(
      () => updateCurrentRomanSunTime(locationDetails, locationName),
      Math.min(Math.ceil(intervalLength * 1000 || 60000), 60000)
    );
  })
  .catch((e) => {
    setNotification(
      "Cannot get your location. This clock cannot work without this information. Please refresh the page."
    );
    console.log(e);
  });
const rootElement = document.getElementById("scRoot");
const FLIPPED_CLASS = "RomanSunclock--flipped";
rootElement.addEventListener("click", () => {
  if (rootElement.classList.contains(FLIPPED_CLASS)) {
    rootElement.classList.remove(FLIPPED_CLASS);
  } else {
    rootElement.classList.add(FLIPPED_CLASS);
  }
});
