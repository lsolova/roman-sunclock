import { asNumber, formatClockTime } from "./src/converters";
import { getLocation, loadLocationDetails } from "./src/location";
import { getLocationName, loadLocationName } from "./src/location-name";
import { Location } from "./src/types";
import { roman_sun_time as romanSunTime } from "roman-sunclock";

function setElementContent(elem: Element | null, content: string) {
  if (elem) {
    elem.innerHTML = content;
  }
}

function setNotification(notification: string): void {
  const notificationE = document.querySelector(".RomanSunclock__Notification");
  setElementContent(notificationE, notification);
}

function updateCurrentRomanSunTime(
  locationDetails: Location,
  locationName: string | null
) {
  const nowDate = new Date();

  const { time_details: romanSunTimeDetails, clock_svg: clockSvg } =
    romanSunTime(
      BigInt(nowDate.getTime()),
      BigInt(nowDate.getTimezoneOffset()),
      locationDetails.lat,
      locationDetails.lon,
      locationDetails.alt
    );

  const lastSunChangeEpoch = asNumber(romanSunTimeDetails.last_sun_change);
  const nextSunChangeEpoch = asNumber(romanSunTimeDetails.next_sun_change);

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
  setElementContent(
    locationDetailsElement,
    `
    ${localNameElement}<span>${new Intl.NumberFormat("en", {
      maximumFractionDigits: 2,
    }).format(romanSunTimeDetails.minute_length)} secs/min</span>
  <span>${
    romanSunTimeDetails.is_day ? "day" : "night"
  }time: ${lastSunChangeTime} - ${nextSunChangeTime}</span>
  `
  );

  const dayClockElement = document.getElementById("dayClockImage");
  setElementContent(dayClockElement, clockSvg);

  const rootE = document.getElementById("scRoot");
  rootE?.classList.remove("RomanSunclock--loading");
  return romanSunTimeDetails.minute_length;
}

navigator.permissions
  .query({ name: "geolocation" })
  .then(({ state }) => {
    if (state === "granted") {
      const locationDetails = loadLocationDetails();
      if (locationDetails) {
        const locationName = loadLocationName(
          locationDetails.lat,
          locationDetails.lon
        );
        updateCurrentRomanSunTime(locationDetails, locationName);
      } else {
        setNotification("Requesting your location. Please, wait.");
      }
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
rootElement?.addEventListener("click", () => {
  const FLIPPED_CLASS = "RomanSunclock--flipped";
  if (rootElement?.classList.contains(FLIPPED_CLASS)) {
    rootElement?.classList.remove(FLIPPED_CLASS);
  } else {
    rootElement?.classList.add(FLIPPED_CLASS);
  }
});
