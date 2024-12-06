const HASH_SEPARATOR = "|";

export const getLocationHash = (): string[] =>
  window.location.hash
    ? window.location.hash.substring(1).split(HASH_SEPARATOR)
    : [];
