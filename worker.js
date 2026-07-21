import init, { Org } from "./dist/orgize.js";
import { applySyncMessage } from "./worker-sync.js";

const sessions = new Map();

let ready;

const ensureReady = (wasmUrl) => {
  if (!ready) {
    ready = init(wasmUrl);
  }
  return ready;
};

const sessionIdFor = (message) => message.sessionId || "default";

const requireSession = (sessionId) => {
  const session = sessions.get(sessionId);
  if (!session) {
    throw new Error(`orgize session '${sessionId}' does not exist`);
  }
  return session;
};

const parseJson = (json) => JSON.parse(json);

const createSession = (source) => {
  const org = new Org(source);
  return {
    org,
    revision: 1,
    sourceLengthBytes: org.sourceLenBytes(),
  };
};

const sessionMetadata = (session, changed) => ({
  revision: session.revision,
  changed,
  sourceLengthBytes: session.sourceLengthBytes,
});

const projectionFor = (
  org,
  projection,
  sourceFile,
  includeBaseDir,
  sparseTreeMatch,
  sparseTreeText,
  sparseTreeIncludeArchived,
  agendaView,
  agendaBlock,
  capturePlan,
  orgElementsIndex,
  clockIssueProfile,
  memory,
  attachmentInventory,
  propertySchemaRegistry
) => {
  const schemaRegistryRequest = () =>
    JSON.stringify(propertySchemaRegistry ?? { contracts: [] });

  switch (projection) {
    case "outline":
      return parseJson(org.outlineJson());
    case "metadata":
      return parseJson(org.metadataJson());
    case "orgElements":
      return parseJson(org.orgElementsJson());
    case "orgInteractive":
      return parseJson(org.orgInteractiveJson());
    case "orgElementsIndex":
      if (orgElementsIndex) {
        return parseJson(org.orgElementsIndexQueryJson(JSON.stringify(orgElementsIndex)));
      }
      return parseJson(org.orgElementsIndexJson());
    case "lint":
      return parseJson(org.lintJson());
    case "sectionIndex":
      return parseJson(org.sectionIndexJson(sourceFile ?? undefined));
    case "sparseTree":
      return parseJson(
        org.sparseTreeJson(
          sourceFile ?? undefined,
          sparseTreeMatch ?? undefined,
          sparseTreeText ?? undefined,
          sparseTreeIncludeArchived
        )
      );
    case "agendaView":
      if (!agendaView || !agendaView.start || !agendaView.end) {
        throw new Error("agendaView projection requires start and end dates");
      }
      return parseJson(org.agendaViewJson(JSON.stringify(agendaView)));
    case "agendaBlock":
      if (!agendaBlock || !Array.isArray(agendaBlock.sections)) {
        throw new Error("agendaBlock projection requires sections");
      }
      return parseJson(org.agendaBlockJson(JSON.stringify(agendaBlock)));
    case "capturePlan":
      if (!capturePlan || !capturePlan.kind || !capturePlan.title) {
        throw new Error("capturePlan projection requires kind and title");
      }
      return parseJson(org.capturePlanJson(JSON.stringify(capturePlan)));
    case "clockIssues":
      return parseJson(
        org.clockIssuesJson(
          clockIssueProfile ? JSON.stringify(clockIssueProfile) : undefined
        )
      );
    case "memory":
      return parseJson(org.memoryJson(memory ? JSON.stringify(memory) : undefined));
    case "propertyProfile":
      return parseJson(org.propertyProfileJson());
    case "propertyProfileWithSchemas":
      return parseJson(org.propertyProfileWithSchemasJson(schemaRegistryRequest()));
    case "crypt":
      return parseJson(org.cryptJson());
    case "runtimeMetadata":
      return parseJson(org.runtimeMetadataJson());
    case "sdd":
      return parseJson(org.sddJson());
    case "viewIndex":
      return parseJson(org.viewIndexJson(sourceFile ?? undefined));
    case "attachments":
      return parseJson(org.attachmentsJson(sourceFile ?? undefined));
    case "attachmentInventory":
      return parseJson(
        org.attachmentInventoryJson(
          attachmentInventory ? JSON.stringify(attachmentInventory) : undefined
        )
      );
    case "sourceBlocks":
      return parseJson(org.sourceBlocksJson());
    case "columnViews":
      return parseJson(org.columnViewsJson());
    case "includeExpansion":
      return parseJson(org.includeExpansionJson(includeBaseDir ?? undefined));
    case "datetree":
      return parseJson(org.datetreeJson());
    case "snapshot":
      if (propertySchemaRegistry) {
        return parseJson(
          org.snapshotWithSchemasJson(schemaRegistryRequest(), sourceFile ?? undefined)
        );
      }
      return parseJson(org.snapshotJson(sourceFile ?? undefined));
    case "snapshotWithSchemas":
      return parseJson(
        org.snapshotWithSchemasJson(schemaRegistryRequest(), sourceFile ?? undefined)
      );
    default:
      throw new Error(`unknown orgize projection '${projection}'`);
  }
};

const projectMessage = (org, message) =>
  projectionFor(
    org,
    message.projection || "snapshot",
    message.sourceFile,
    message.includeBaseDir,
    message.sparseTreeMatch,
    message.sparseTreeText,
    message.sparseTreeIncludeArchived,
    message.agendaView,
    message.agendaBlock,
    message.capturePlan,
    message.orgElementsIndex,
    message.clockIssueProfile,
    message.memory,
    message.attachmentInventory,
    message.propertySchemaRegistry
  );

const renderFor = (org, format) => {
  switch (format) {
    case "html":
      return org.html();
    case "latex":
      return org.latex();
    case "markdown":
      return org.markdown();
    case "org":
      return org.org();
    case "syntax":
      return org.syntax();
    case "semantic":
      return org.semantic();
    case "agenda":
      return org.agenda();
    case "agentPlanning":
      return org.agentPlanning();
    case "agentMemory":
      return org.agentMemory();
    case "sdd":
      return org.sdd();
    case "traverse":
      return org.traverse();
    default:
      throw new Error(`unknown orgize render format '${format}'`);
  }
};

const dispose = (sessionId) => {
  const session = sessions.get(sessionId);
  if (session) {
    session.org.free();
    sessions.delete(sessionId);
  }
};

self.addEventListener("message", (event) => {
  void handleMessage(event.data);
});

const handleMessage = async (message) => {
  const command = message.command ?? message.type;
  const sessionId = sessionIdFor(message);
  try {
    if (command !== "init") {
      await ensureReady(message.wasmUrl);
    }

    let result;
    let metadata;
    switch (command) {
      case "init": {
        await ensureReady(message.wasmUrl);
        result = {
          buildTime: Org.buildTime,
          gitHash: Org.gitHash,
        };
        break;
      }
      case "parse": {
        if (typeof message.source !== "string") {
          throw new Error("orgize parse requires a source string");
        }
        dispose(sessionId);
        const session = createSession(message.source);
        sessions.set(sessionId, session);
        result = projectMessage(session.org, message);
        metadata = sessionMetadata(session, true);
        break;
      }
      case "update": {
        if (typeof message.source !== "string") {
          throw new Error("orgize update requires a source string");
        }
        let session = sessions.get(sessionId);
        if (!session) {
          session = createSession(message.source);
          sessions.set(sessionId, session);
        } else {
          session.org.update(message.source);
          session.revision += 1;
          session.sourceLengthBytes = session.org.sourceLenBytes();
        }
        result = projectMessage(session.org, message);
        metadata = sessionMetadata(session, true);
        break;
      }
      case "sync": {
        const session = requireSession(sessionId);
        const changed = applySyncMessage(session, message);
        result = projectMessage(session.org, message);
        metadata = sessionMetadata(session, changed);
        break;
      }
      case "projection": {
        const session = requireSession(sessionId);
        result = projectMessage(session.org, message);
        break;
      }
      case "render": {
        result = renderFor(requireSession(sessionId).org, message.format || "html");
        break;
      }
      case "format": {
        const session = requireSession(sessionId);
        result = parseJson(
          session.org.format(message.options ? JSON.stringify(message.options) : undefined)
        );
        break;
      }
      case "dispose": {
        dispose(sessionId);
        result = { disposed: sessionId };
        break;
      }
      case "disposeAll": {
        for (const key of [...sessions.keys()]) {
          dispose(key);
        }
        result = { disposed: "all" };
        break;
      }
      default:
        throw new Error(`unknown orgize worker command '${command}'`);
    }

    self.postMessage({
      type: "result",
      id: message.id,
      requestId: message.requestId,
      command,
      sessionId,
      ok: true,
      result,
      ...(metadata || {}),
    });
  } catch (error) {
    self.postMessage({
      type: "error",
      id: message.id,
      requestId: message.requestId,
      command,
      sessionId,
      ok: false,
      error: {
        name: error && error.name ? error.name : "Error",
        message: error && error.message ? error.message : String(error),
        stack: error && error.stack ? error.stack : undefined,
      },
    });
  }
};
