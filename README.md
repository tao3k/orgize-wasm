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

Pinned git consumption uses the generated `npm-package` branch. Prefer a
specific commit from that branch so lockfiles point at immutable build output:

```json
{
  "dependencies": {
    "orgize": "git+https://github.com/tao3k/orgize-wasm.git#<npm-package-commit>"
  }
}
```

The branch name also works for local experiments:

```json
{
  "dependencies": {
    "orgize": "github:tao3k/orgize-wasm#npm-package"
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
`snapshot`, `outline`, `orgInteractive`, `sectionIndex`, `attachments`, and
`lint`.

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
When a host has loaded property-schema contracts, pass
`propertySchemaRegistry` with `projection: "snapshot"` or
`projection: "snapshotWithSchemas"` so the returned `propertyProfile` and
`lint` entries are validated against the same registry.
For `projection: "orgElementsIndex"`, pass `orgElementsIndex.summaryEquals`
or `orgElementsIndex.summaryContains` to filter compact index rows by fields
such as link `path`, section `title`, or source-block `language` without
shipping the full Org elements tree back to the UI.

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
   `orgInteractiveJson`,
   `sparseTreeJson`, `agendaViewJson`, `agendaBlockJson`,
   `capturePlanJson`, `attachmentsJson`, `sourceBlocksJson`,
   `columnViewsJson`, `propertyProfileJson`, `propertyProfileWithSchemasJson`,
   `includeExpansionJson`, `datetreeJson`, `snapshotJson`, and
   `snapshotWithSchemasJson`. `orgInteractiveJson` validates formal
   `org-contract :type agent-interactive` blocks with the Rust-owned parser
   before returning typed choice DTOs. `sourceBlocksJson` includes block records with
   `#+PROPERTY: header-args`, `#+HEADER:`, and `#+BEGIN_SRC` header arguments
   plus non-executing tangle metadata for `:mkdirp`, `:comments`, `:shebang`,
   and `:noweb`, plus result planning metadata for `:results` and `:file`
   output hints, and execution/export planning metadata for `:eval`,
   `:exports`, `:cache`, `:session`, `:dir`, `:hlines`, and context-specific
   `:noweb`. It also exposes literate references for `#+CALL`, inline
   `call_name(...)`, source-block `:var` dependencies, and noweb `<<name>>`
   targets. Worker users receive parsed objects;

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

`main` is the source branch. Generated `wasm-pack` output is intentionally kept
out of `main`; GitHub Actions builds it after source validation and appends the
git-consumable payload to the `npm-package` branch. That branch carries
`.orgize-source-rev` and `.orgize-parent-rev` so a package commit can be traced
back to both the standalone wasm source commit and the parent parser checkout
used for the build.

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
