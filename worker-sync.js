const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder("utf-8", { fatal: true });

const patchOffset = (patch, key) => {
  const value = patch?.[key];
  if (!Number.isInteger(value) || value < 0) {
    throw new Error(`orgize sync patch ${key} must be a non-negative integer`);
  }
  return value;
};

const applyTextPatch = (source, patch) => {
  const start = patchOffset(patch, "start");
  const end = patchOffset(patch, "end");
  if (start > end) {
    throw new Error("orgize sync patch start must be <= end");
  }
  if (typeof patch.text !== "string") {
    throw new Error("orgize sync patch text must be a string");
  }

  const sourceBytes = textEncoder.encode(source);
  if (end > sourceBytes.length) {
    throw new Error("orgize sync patch range out of bounds");
  }

  const textBytes = textEncoder.encode(patch.text);
  const nextBytes = new Uint8Array(start + textBytes.length + sourceBytes.length - end);
  nextBytes.set(sourceBytes.subarray(0, start), 0);
  nextBytes.set(textBytes, start);
  nextBytes.set(sourceBytes.subarray(end), start + textBytes.length);

  try {
    return textDecoder.decode(nextBytes);
  } catch (_error) {
    throw new Error("orgize sync patch range must align to UTF-8 character boundaries");
  }
};

export const applyTextPatches = (source, patches) => {
  if (!Array.isArray(patches) || patches.length === 0) {
    throw new Error("orgize sync requires at least one patch");
  }
  return patches.reduce((currentSource, patch) => applyTextPatch(currentSource, patch), source);
};

export const applySyncMessage = (session, message) => {
  if (!Number.isInteger(message.revision)) {
    throw new Error("orgize sync requires a revision");
  }
  if (message.revision !== session.revision) {
    throw new Error(
      `orgize sync revision mismatch: expected ${session.revision}, received ${message.revision}`
    );
  }

  const nextSource = applyTextPatches(session.source, message.patches);
  const changed = nextSource !== session.source;
  for (const patch of message.patches) {
    session.org.replaceRange(patch.start, patch.end, patch.text);
  }
  session.source = nextSource;
  session.revision += 1;
  return changed;
};
