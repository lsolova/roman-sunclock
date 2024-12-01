function saveLocationName(
  lat: number,
  lon: number,
  locationName: string
): void {
  window.localStorage.setItem(
    "rsctLocationName",
    JSON.stringify({
      lat,
      lon,
      locationName,
    })
  );
}

export function loadLocationName(lat: number, lon: number): string | null {
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

export function getLocationName(
  lat: number,
  lon: number
): Promise<string | null> {
  return new Promise((resolve) => {
    const locationName = loadLocationName(lat, lon);
    if (locationName) {
      resolve(locationName);
      return;
    }
    fetch(
      `https://api.geoapify.com/v1/geocode/reverse?lat=${lat}&lon=${lon}&type=city&limit=1&format=json&apiKey=${window.reverseGeocodeApiKey}`
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
