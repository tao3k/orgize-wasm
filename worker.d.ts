import type {
  OrgizeAgendaBlockJsonRequestDto,
  OrgizeAgendaViewJsonRequestDto,
  OrgizeAgentCaptureRequestDto,
  OrgizeAttachmentInventoryRequestDto,
  OrgizeClockIssueProfileRequestDto,
  OrgizeMemoryJsonRequestDto,
  OrgizeOrgElementsIndexQueryDto,
  OrgizeProjectionDto,
  OrgizeProjectionName,
  OrgizePropertySchemaRegistryRequestDto,
} from "./dto.js";

export type OrgizeRenderFormat =
  | "html"
  | "latex"
  | "markdown"
  | "org"
  | "syntax"
  | "semantic"
  | "agenda"
  | "agentPlanning"
  | "agentMemory"
  | "sdd"
  | "traverse";

export interface OrgizeTextPatch {
  /** UTF-8 byte offset in the current Org source. */
  start: number;
  /** UTF-8 byte offset in the current Org source. */
  end: number;
  text: string;
}

export interface OrgizeFormatOptionsDto {
  trimTrailingWhitespace?: boolean;
  alignTables?: boolean;
  finalNewline?: boolean;
}

export interface OrgizeFormatResponseDto {
  schemaVersion: 1;
  output: string;
  changed: boolean;
}

export interface OrgizeWorkerBaseRequest {
  id?: string | number;
  requestId?: string | number;
  sessionId?: string;
  wasmUrl?: string | URL | Request;
  sparseTreeMatch?: string;
  sparseTreeText?: string;
  sparseTreeIncludeArchived?: boolean;
  agendaView?: OrgizeAgendaViewJsonRequestDto;
  agendaBlock?: OrgizeAgendaBlockJsonRequestDto;
  capturePlan?: OrgizeAgentCaptureRequestDto;
  orgElementsIndex?: OrgizeOrgElementsIndexQueryDto;
  clockIssueProfile?: OrgizeClockIssueProfileRequestDto;
  memory?: OrgizeMemoryJsonRequestDto;
  attachmentInventory?: OrgizeAttachmentInventoryRequestDto;
  propertySchemaRegistry?: OrgizePropertySchemaRegistryRequestDto;
}

export interface OrgizeWorkerInitRequest extends OrgizeWorkerBaseRequest {
  command: "init";
}

export interface OrgizeWorkerParseRequest extends OrgizeWorkerBaseRequest {
  command: "parse";
  source: string;
  sourceFile?: string;
  includeBaseDir?: string;
  projection?: OrgizeProjectionName;
}

export interface OrgizeWorkerUpdateRequest extends OrgizeWorkerBaseRequest {
  command: "update";
  source: string;
  sourceFile?: string;
  includeBaseDir?: string;
  projection?: OrgizeProjectionName;
}

export interface OrgizeWorkerSyncRequest extends OrgizeWorkerBaseRequest {
  command: "sync";
  revision: number;
  patches: OrgizeTextPatch[];
  sourceFile?: string;
  includeBaseDir?: string;
  projection?: OrgizeProjectionName;
}

export interface OrgizeWorkerProjectionRequest extends OrgizeWorkerBaseRequest {
  command: "projection";
  sourceFile?: string;
  includeBaseDir?: string;
  projection?: OrgizeProjectionName;
}

export interface OrgizeWorkerRenderRequest extends OrgizeWorkerBaseRequest {
  command: "render";
  format?: OrgizeRenderFormat;
}

export interface OrgizeWorkerFormatRequest extends OrgizeWorkerBaseRequest {
  command: "format";
  options?: OrgizeFormatOptionsDto;
}

export interface OrgizeWorkerDisposeRequest extends OrgizeWorkerBaseRequest {
  command: "dispose" | "disposeAll";
}

export type OrgizeWorkerRequest =
  | OrgizeWorkerInitRequest
  | OrgizeWorkerParseRequest
  | OrgizeWorkerUpdateRequest
  | OrgizeWorkerSyncRequest
  | OrgizeWorkerProjectionRequest
  | OrgizeWorkerRenderRequest
  | OrgizeWorkerFormatRequest
  | OrgizeWorkerDisposeRequest;

export interface OrgizeWorkerResultMessage<T = OrgizeProjectionDto | OrgizeFormatResponseDto | string> {
  type: "result";
  id?: string | number;
  requestId?: string | number;
  command: OrgizeWorkerRequest["command"];
  sessionId: string;
  ok: true;
  revision?: number;
  changed?: boolean;
  sourceLengthBytes?: number;
  result: T;
}

export interface OrgizeWorkerErrorMessage {
  type: "error";
  id?: string | number;
  requestId?: string | number;
  command?: string;
  sessionId: string;
  ok: false;
  error: {
    name: string;
    message: string;
    stack?: string;
  };
}

export type OrgizeWorkerMessage<T = OrgizeProjectionDto | string> =
  | OrgizeWorkerResultMessage<T>
  | OrgizeWorkerErrorMessage;
