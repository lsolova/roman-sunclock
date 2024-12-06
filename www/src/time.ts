import { getLocationHash } from "./utils";

const VALID_TIME_REGEX = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/;

export const getRequestedDate = () => {
  const locationHash = getLocationHash();
  return locationHash.length >= 4 && VALID_TIME_REGEX.test(locationHash[3])
    ? new Date(locationHash[3])
    : new Date();
};
