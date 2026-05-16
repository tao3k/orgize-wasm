# orgize

![npm](https://img.shields.io/npm/v/orgize)

This package lives in the standalone
[`tao3k/orgize-wasm`](https://github.com/tao3k/orgize-wasm) repository. In the
Rust parser repository it is mounted at `wasm/` as a git submodule so the
browser and npm boundary can move independently from parser internals.

## Install

```sh
npm install orgize
yarn add orgize
```

Pinned git consumption also works because `dist/` is intentionally committed:

```json
{
  "dependencies": {
    "orgize": "github:tao3k/orgize-wasm#<commit-sha>"
  }
}
```

## Browser

```js
import init, { Org } from "orgize";

init().then(() => {
  const org = new Org("* Hello, /world/!");
  const html = org.html();
  console.log(html);
  org.free();
});
```

## Browser Worker

Use the worker entry when large documents must not block React rendering. The
worker owns the Rust parser session and returns typed DTO projections such as
`snapshot`, `outline`, `sectionIndex`, `attachments`, and `lint`.

```ts
import type { OrgizeWorkerMessage, OrgizeWorkerRequest } from "orgize/worker";

const worker = new Worker(new URL("orgize/worker", import.meta.url), {
  type: "module",
});

worker.onmessage = (event: MessageEvent<OrgizeWorkerMessage>) => {
  if (!event.data.ok) {
    console.error(event.data.error.message);
    return;
  }

  console.log(event.data.result);
};

worker.postMessage({
  command: "parse",
  requestId: "initial",
  sessionId: "editor",
  projection: "snapshot",
  source: "* NEXT Ship WASM DTOs\nSCHEDULED: <2026-05-14 Thu>\n",
} satisfies OrgizeWorkerRequest);
```

For TanStack Query or Effect integrations, treat the worker response DTOs as the
cache boundary. Keep the full semantic AST in Rust and request smaller
projections for UI panels.

## Node.js

```js
import { Org, initSync } from "orgize";
import { readFile } from "node:fs/promises";

// you can also use import.meta.resolve, but it's currently behind
// an experimental flag --experimental-import-meta-resolve
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);

readFile(require.resolve("orgize/wasm")).then((bytes) => {
  initSync(bytes);

  const org = new Org("* Hello, /world/!");
  const html = org.html();
  console.log(html);
  org.free();
});
```

## Notes

1. You must **initialize** the WebAssembly module (using either `init` or
   `initSync` function) before using the `Org` class;

2. Don't forget to call `org.free()` to **release the memory** that
   allocated by Rust;

3. The worker owns session disposal through `dispose` and `disposeAll`
   commands. Direct `Org` users still own `free()`;

4. The WebAssembly API exposes projection methods that return JSON strings:
   `outlineJson`, `metadataJson`, `lintJson`, `sectionIndexJson`,
   `sparseTreeJson`, `agendaViewJson`, `agendaBlockJson`,
   `capturePlanJson`, `attachmentsJson`, `sourceBlocksJson`,
   `columnViewsJson`, `includeExpansionJson`, `datetreeJson`, and
   `snapshotJson`. Worker users receive parsed objects;

5. This npm package is primarily aimed at browser demos and high-level
   integrations. If you need a custom Node-only package, build with `napi`.

## Development

`orgize-wasm` expects the Rust parser checkout as its parent directory during
local development and CI:

```text
orgize/
  Cargo.toml
  wasm/  # this repository
```

That layout keeps the crate dependency as `orgize = { path = ".." }` for fast
parser-branch iteration. The standalone GitHub Actions workflow checks out
`tao3k/orgize` first, replaces `orgize/wasm` with this repository, then runs the
Rust tests and `wasm-pack` build there.

The generated `dist/` directory is part of the repository contract, not a local
cache. Commit it whenever bindings or DTO types change so downstream
`package.json` git dependencies can import the package without running Rust or
`wasm-pack` locally.

Useful local commands:

```sh
just fmt
just test
just clippy
just build
just pack
```

## License

MIT
