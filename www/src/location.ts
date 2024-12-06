import { Location } from "./types";
import { getLocationHash } from "./utils";

function saveLocationDetails(locationDetails: Location) {
  window.localStorage.setItem(
    "rsctLocationDetails",
    JSON.stringify({
      ...locationDetails,
      expiry: Date.now() + 86400000,
    })
  );
}

export function loadLocationDetails(): Location | null {
  const locationDetailsString = window.localStorage.getItem(
    "rsctLocationDetails"
  );
  if (locationDetailsString) {
    const locationDetails = JSON.parse(locationDetailsString);
    if (Date.now() < locationDetails.expiry) {
      return locationDetails;
    }
  }
  return null;
}

const getLocationByHash = (): Location | null => {
  const sp = getLocationHash();
  if (sp.length >= 3) {
    const lat = Number.parseFloat(sp[0]);
    const lon = Number.parseFloat(sp[1]);
    const alt = Number.parseFloat(sp[2]);
    if (
      -90 <= lat &&
      lat <= 90 &&
      -180 <= lon &&
      lon <= 180 &&
      0 <= alt &&
      alt <= 5000
    ) {
      return { lat, lon, alt, source: "hash" };
    }
  }
  return null;
};

export function getLocation(): Promise<Location> {
  return new Promise((resolve, reject) => {
    const location = getLocationByHash();
    if (location !== null) {
      resolve(location);
      return;
    }

    const options = {
      enableHighAccuracy: true,
      timeout: 5000,
      maximumAge: 0,
    };

    function success(pos: GeolocationPosition) {
      const crd = pos.coords;

      const locationDetails: Location = {
        lat: crd.latitude,
        lon: crd.longitude,
        alt: crd.altitude || 0,
        source: "gps",
      };

      saveLocationDetails(locationDetails);
      resolve(locationDetails);
    }

    function error(err: GeolocationPositionError) {
      const locationDetails = loadLocationDetails();
      if (locationDetails) {
        resolve(locationDetails);
        return;
      }
      reject(new Error(`ERROR(${err.code}): ${err.message}`));
    }

    navigator.geolocation.getCurrentPosition(success, error, options);
  });
}
