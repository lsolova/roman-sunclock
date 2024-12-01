import { Location } from "./types";

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

export function getLocation(): Promise<Location> {
  return new Promise((resolve, reject) => {
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
