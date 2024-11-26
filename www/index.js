import * as wasm from "roman-sunclock";

const CANVAS_X_CENTER = 125;
const CANVAS_Y_CENTER = 125;

function asNumber(bigint) {
  return Number(bigint.toString());
}

function formatClockTime(hours, minutes) {
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}`;
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
  const nowTime = new Date();
  const romanSunTime = wasm.roman_sun_time(
    BigInt(nowTime.getTime()),
    locationDetails.lat,
    locationDetails.lon,
    locationDetails.alt
  );
  const romanSunclockTime = formatClockTime(
    romanSunTime.hours,
    romanSunTime.minutes
  );
  const todayStartEpoch = asNumber(romanSunTime.today_start);
  const todaySunriseEpoch = asNumber(romanSunTime.today_sunrise);
  const todaySunsetEpoch = asNumber(romanSunTime.today_sunset);
  const todaySunriseTime = formatClockTime(
    new Date(todaySunriseEpoch).getHours(),
    new Date(todaySunriseEpoch).getMinutes()
  );
  const todaySunsetTime = formatClockTime(
    new Date(todaySunsetEpoch).getHours(),
    new Date(todaySunsetEpoch).getMinutes()
  );

  const dayOrNightElement = document.getElementById("dayNightIcon");
  dayOrNightElement.classList.remove(
    `RomanSunclock__DayOrNight--${romanSunTime.is_day ? "night" : "day"}`
  );
  dayOrNightElement.classList.add(
    `RomanSunclock__DayOrNight--${romanSunTime.is_day ? "day" : "night"}`
  );

  const nowTimeElement = document.getElementById("scTime");
  nowTimeElement.innerText = romanSunclockTime;

  const locationDetailsElement = document.getElementById("scDetails");
  const localNameElement = locationName ? `<span>${locationName}</span>` : "";
  locationDetailsElement.innerHTML = `
    <span>${formatClockTime(
      nowTime.getHours(),
      nowTime.getMinutes()
    )}</span>${localNameElement}<span>${new Intl.NumberFormat("en", {
    maximumFractionDigits: 2,
  }).format(romanSunTime.minute_length)} secs/min</span>
  `;

  const calculationDetailsElement = document.getElementById("scCalcDetails");
  calculationDetailsElement.innerHTML = `<span>sunrise: ${todaySunriseTime}</span><span>sunset: ${todaySunsetTime}</span>`;

  const dataCanvasElement = document.getElementById("dayNightInfoCanvas");
  if (dataCanvasElement.getContext) {
    const mainColor = window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "white"
      : "#333";
    const ctx = dataCanvasElement.getContext("2d");
    ctx.reset();
    const timezoneOffset = new Date().getTimezoneOffset() * 60 * 1000;
    const sunriseEpochDiff =
      todaySunriseEpoch - todayStartEpoch - timezoneOffset;
    const sunsetEpochDiff = todaySunsetEpoch - todayStartEpoch - timezoneOffset;
    const nowDiff = nowTime - todayStartEpoch - timezoneOffset;
    const sunriseW = (sunriseEpochDiff / 86400000) * Math.PI * 2 - Math.PI / 2;
    const sunsetW = (sunsetEpochDiff / 86400000) * Math.PI * 2 - Math.PI / 2;
    const nowW = (nowDiff / 86400000) * Math.PI * 2 - Math.PI / 2;

    ctx.strokeStyle = "#333";
    ctx.lineWidth = 30;
    ctx.beginPath();
    ctx.arc(CANVAS_X_CENTER, CANVAS_Y_CENTER, 105, -(Math.PI / 2), sunriseW);
    ctx.stroke();
    ctx.strokeStyle = "white";
    ctx.beginPath();
    ctx.arc(CANVAS_X_CENTER, CANVAS_Y_CENTER, 105, sunriseW, sunsetW);
    ctx.stroke();
    ctx.strokeStyle = "#333";
    ctx.beginPath();
    ctx.arc(CANVAS_X_CENTER, CANVAS_Y_CENTER, 105, sunsetW, Math.PI * 1.5);
    ctx.stroke();

    ctx.strokeStyle = "white";
    ctx.lineWidth = 1;

    const step = (Math.PI * 2 - (sunsetW - sunriseW)) / 12;
    for (let i = 1; i < 12; i++) {
      const w = sunsetW + i * step;
      const length = i % 3 === 0 ? 107 : 113;
      const x1 = length * Math.cos(w) + CANVAS_X_CENTER;
      const y1 = length * Math.sin(w) + CANVAS_Y_CENTER;
      const x2 = 120 * Math.cos(w) + CANVAS_X_CENTER;
      const y2 = 120 * Math.sin(w) + CANVAS_Y_CENTER;
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);
      ctx.stroke();
    }

    ctx.strokeStyle = "#333";
    const step2 = (sunsetW - sunriseW) / 12;
    for (let i = 1; i < 12; i++) {
      const w = sunriseW + i * step2;
      const length = i % 3 === 0 ? 107 : 113;
      const x1 = length * Math.cos(w) + CANVAS_X_CENTER;
      const y1 = length * Math.sin(w) + CANVAS_Y_CENTER;
      const x2 = 120 * Math.cos(w) + CANVAS_X_CENTER;
      const y2 = 120 * Math.sin(w) + CANVAS_Y_CENTER;
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);
      ctx.stroke();
    }

    ctx.strokeStyle = mainColor;
    ctx.beginPath();
    ctx.arc(CANVAS_X_CENTER, CANVAS_Y_CENTER, 90, 0, Math.PI * 2, false);
    ctx.stroke();
    ctx.closePath();
    ctx.beginPath();
    ctx.arc(CANVAS_X_CENTER, CANVAS_Y_CENTER, 120, 0, Math.PI * 2, false);
    ctx.stroke();
    ctx.closePath();
    for (let i = 0; i < 24; i++) {
      const w = (i * Math.PI) / 12;
      const x1 = 90 * Math.sin(w) + CANVAS_X_CENTER;
      const y1 = 90 * Math.cos(w) + CANVAS_Y_CENTER;
      ctx.moveTo(x1, y1);
      const length = i % 3 === 0 ? 80 : 85;
      const x2 = length * Math.sin(w) + CANVAS_X_CENTER;
      const y2 = length * Math.cos(w) + CANVAS_Y_CENTER;
      ctx.lineTo(x2, y2);
      ctx.stroke();
    }

    ctx.fillStyle = mainColor;
    ctx.textAlign = "center";
    ctx.font = "32px serif";
    ctx.fillText(romanSunclockTime, CANVAS_X_CENTER, 120);
    ctx.fillStyle = "#333";
    ctx.fillText(formatClockTime(
      nowTime.getHours(),
      nowTime.getMinutes()
    ), CANVAS_X_CENTER, 152);

    ctx.fillStyle = "red";
    const arcX = 102 * Math.cos(nowW) + CANVAS_X_CENTER;
    const arcY = 102 * Math.sin(nowW) + CANVAS_Y_CENTER;
    ctx.beginPath();
    ctx.arc(arcX, arcY, 4, 0, Math.PI * 2);
    ctx.fill();
  }

  const faceList = document.getElementsByClassName("RomanSunclock");
  for (let i = 0; i < faceList.length; i++) {
    faceList.item(i).classList.remove("RomanSunclock--loading");
  }
  return romanSunTime.minute_length;
}

navigator.permissions.query({ name: "geolocation" }).then(({ state }) => {
  if (state === "granted") {
    setNotification("Requesting your location. Please, wait.");
  } else {
    setNotification(
      "Please, allow location services to get your local Roman Sunclock Time."
    );
  }
});
getLocation()
  .then((locationDetails) => {
    const { lat, lon } = locationDetails;
    getLocationName(lat, lon).then((locationName) => {
      const intervalLength = updateCurrentRomanSunTime(
        locationDetails,
        locationName
      );
      setInterval(
        () => updateCurrentRomanSunTime(locationDetails, locationName),
        intervalLength * 1000 || 1000
      );
    });
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
