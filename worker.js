import init, { Org } from "./dist/orgize.js";

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
  const org = sessions.get(sessionId);
  if (!org) {
    throw new Error(`orgize session '${sessionId}' does not exist`);
  }
  return org;
};

const parseJson = (json) => JSON.parse(json);

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
  clockIssueProfile
) => {
  switch (projection) {
    case "outline":
      return parseJson(org.outlineJson());
    case "metadata":
      return parseJson(org.metadataJson());
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
    case "clockIssues":
      return parseJson(
        org.clockIssuesJson(
          clockIssueProfile ? JSON.stringify(clockIssueProfile) : undefined
        )
      );
    case "viewIndex":
      return parseJson(org.viewIndexJson(sourceFile ?? undefined));
    case "attachments":
      return parseJson(org.attachmentsJson(sourceFile ?? undefined));
    case "sourceBlocks":
      return parseJson(org.sourceBlocksJson());
    case "columnViews":
      return parseJson(org.columnViewsJson());
    case "includeExpansion":
      return parseJson(org.includeExpansionJson(includeBaseDir ?? undefined));
    case "datetree":
      return parseJson(org.datetreeJson());
    case "snapshot":
      return parseJson(org.snapshotJson(sourceFile ?? undefined));
    default:
      throw new Error(`unknown orgize projection '${projection}'`);
  }
};

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
    case "traverse":
      return org.traverse();
    default:
      throw new Error(`unknown orgize render format '${format}'`);
  }
};

const dispose = (sessionId) => {
  const org = sessions.get(sessionId);
  if (org) {
    org.free();
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
        const org = new Org(message.source);
        sessions.set(sessionId, org);
        result = projectionFor(
          org,
          message.projection || "snapshot",
          message.sourceFile,
          message.includeBaseDir,
          message.sparseTreeMatch,
          message.sparseTreeText,
          message.sparseTreeIncludeArchived,
          message.agendaView,
          message.agendaBlock,
          message.clockIssueProfile
        );
        break;
      }
      case "update": {
        if (typeof message.source !== "string") {
          throw new Error("orgize update requires a source string");
        }
        let org = sessions.get(sessionId);
        if (!org) {
          org = new Org(message.source);
          sessions.set(sessionId, org);
        } else {
          org.update(message.source);
        }
        result = projectionFor(
          org,
          message.projection || "snapshot",
          message.sourceFile,
          message.includeBaseDir,
          message.sparseTreeMatch,
          message.sparseTreeText,
          message.sparseTreeIncludeArchived,
          message.agendaView,
          message.agendaBlock,
          message.clockIssueProfile
        );
        break;
      }
      case "projection": {
        result = projectionFor(
          requireSession(sessionId),
          message.projection || "snapshot",
          message.sourceFile,
          message.includeBaseDir,
          message.sparseTreeMatch,
          message.sparseTreeText,
          message.sparseTreeIncludeArchived,
          message.agendaView,
          message.agendaBlock,
          message.clockIssueProfile
        );
        break;
      }
      case "render": {
        result = renderFor(requireSession(sessionId), message.format || "html");
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
