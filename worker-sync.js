const textEncoder = new TextEncoder();

const patchOffset = (patch, key) => {
  const value = patch?.[key];
  if (!Number.isInteger(value) || value < 0) {
    throw new Error(`orgize sync patch ${key} must be a non-negative integer`);
  }
  return value;
};

const patchTextLength = (text) => textEncoder.encode(text).length;

const nextSourceLength = (sourceLengthBytes, patch) => {
  const start = patchOffset(patch, "start");
  const end = patchOffset(patch, "end");
  if (start > end) {
    throw new Error("orgize sync patch start must be <= end");
  }
  if (typeof patch.text !== "string") {
    throw new Error("orgize sync patch text must be a string");
  }
  if (end > sourceLengthBytes) {
    throw new Error("orgize sync patch range out of bounds");
  }

  return sourceLengthBytes - (end - start) + patchTextLength(patch.text);
};

export const validateTextPatches = (sourceLengthBytes, patches) => {
  if (!Array.isArray(patches) || patches.length === 0) {
    throw new Error("orgize sync requires at least one patch");
  }
  return patches.reduce(
    (currentLength, patch) => nextSourceLength(currentLength, patch),
    sourceLengthBytes
  );
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

  const nextLength = validateTextPatches(session.sourceLengthBytes, message.patches);
  const changed = message.patches.some((patch) => patch.start !== patch.end || patch.text !== "");
  for (const patch of message.patches) {
    session.org.replaceRange(patch.start, patch.end, patch.text);
  }
  session.sourceLengthBytes = nextLength;
  session.revision += 1;
  return changed;
};
