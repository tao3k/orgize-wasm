export interface OrgizeSourcePositionDto {
  line: number;
  column: number;
}

export interface OrgizeSourceRangeDto {
  start: OrgizeSourcePositionDto;
  end: OrgizeSourcePositionDto;
  rangeStart: number;
  rangeEnd: number;
}

export interface OrgizePriorityDto {
  raw?: string | null;
  effective: string;
  isDefault: boolean;
  score?: number | null;
  rangeStatus: "inRange" | "outOfRange" | "unsupported";
  profile: OrgizePriorityProfileDto;
}

export interface OrgizePriorityProfileDto {
  highest: string;
  lowest: string;
  default: string;
}

export interface OrgizeArchiveDto {
  archived: boolean;
  hasArchiveTag: boolean;
  location?: string | null;
}

export interface OrgizeAttachmentDirectorySourceDto {
  kind: "dirProperty" | "legacyAttachDirProperty" | "idDerived";
  id?: string | null;
  layout?: "uuid" | "timestamp" | "fallback" | null;
}

export interface OrgizeAttachmentDirectoryDto {
  source: OrgizeAttachmentDirectorySourceDto;
  path: string;
}

export interface OrgizeAttachmentStateDto {
  hasAttachTag: boolean;
  directory?: OrgizeAttachmentDirectoryDto | null;
}

export interface OrgizeOutlineNodeDto {
  source: OrgizeSourceRangeDto;
  level: number;
  title: string;
  anchor?: string | null;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  priority: OrgizePriorityDto;
  tags: string[];
  effectiveTags: string[];
  isComment: boolean;
  archive: OrgizeArchiveDto;
  attachment: OrgizeAttachmentStateDto;
  children: OrgizeOutlineNodeDto[];
}

export interface OrgizeOutlineResponseDto {
  schemaVersion: 1;
  nodes: OrgizeOutlineNodeDto[];
}

export interface OrgizeKeywordAttributeDto {
  key: string;
  value?: string | null;
  raw: string;
}

export interface OrgizeKeywordDto {
  source: OrgizeSourceRangeDto;
  key: string;
  optional?: string | null;
  value: string;
  parsedObjectCount: number;
  attributes: OrgizeKeywordAttributeDto[];
}

export interface OrgizeExportSettingsDto {
  selectTags: string[];
  excludeTags: string[];
  headlineLevels?: number | null;
  specialStrings?: boolean | null;
  expandEntities?: boolean | null;
}

export interface OrgizeTagDefinitionDto {
  name: string;
  shortcut?: string | null;
  raw: string;
}

export interface OrgizePropertyDto {
  source: OrgizeSourceRangeDto;
  key: string;
  value: string;
}

export interface OrgizeLinkAbbreviationDto {
  name: string;
  replacement: string;
  rawValue: string;
}

export interface OrgizeIncludeOptionDto {
  key: string;
  value?: string | null;
  raw: string;
}

export interface OrgizeIncludeDirectiveDto {
  source: OrgizeSourceRangeDto;
  path: string;
  rawPath: string;
  arguments: string[];
  options: OrgizeIncludeOptionDto[];
  rawValue: string;
}

export interface OrgizeIncludeLineSelectionDto {
  kind: "all" | "range" | "invalid";
  start?: number | null;
  end?: number | null;
  raw?: string | null;
}

export interface OrgizeIncludeExpansionModeDto {
  kind: "org" | "example" | "source" | "export" | "other";
  language?: string | null;
  backend?: string | null;
  arguments: string[];
}

export interface OrgizeIncludeExpansionEntryDto {
  directive: OrgizeIncludeDirectiveDto;
  resolvedPath?: string | null;
  lineSelection: OrgizeIncludeLineSelectionDto;
  minLevel?: number | null;
  mode: OrgizeIncludeExpansionModeDto;
  options: OrgizeIncludeOptionDto[];
}

export interface OrgizeIncludeExpansionResponseDto {
  schemaVersion: 1;
  entries: OrgizeIncludeExpansionEntryDto[];
}

export interface OrgizeDateTreeEntryDto {
  source: OrgizeSourceRangeDto;
  date: string;
  year: number;
  month: number;
  day: number;
  yearTitle: string;
  monthTitle: string;
  dayTitle: string;
  outlinePath: string[];
}

export interface OrgizeDateTreeResponseDto {
  schemaVersion: 1;
  records: OrgizeDateTreeEntryDto[];
}

export interface OrgizeMacroDefinitionDto {
  source: OrgizeSourceRangeDto;
  name: string;
  template: string;
  rawValue: string;
}

export type OrgizeTargetKindDto =
  | "headline"
  | "customId"
  | "id"
  | "target"
  | "radioTarget"
  | "footnoteDefinition"
  | "codeRef";

export interface OrgizeTargetDefinitionDto {
  source: OrgizeSourceRangeDto;
  kind: OrgizeTargetKindDto;
  key: string;
  value: string;
  raw: string;
  aliasObjectCount: number;
}

export interface OrgizeFootnoteEntryDto {
  source: OrgizeSourceRangeDto;
  label: string;
  definitionKind: "standalone" | "inline";
}

export interface OrgizeMetadataResponseDto {
  schemaVersion: 1;
  properties: OrgizePropertyDto[];
  keywords: OrgizeKeywordDto[];
  filetags: string[];
  tagDefinitions: OrgizeTagDefinitionDto[];
  exportSettings: OrgizeExportSettingsDto;
  linkAbbreviations: OrgizeLinkAbbreviationDto[];
  includes: OrgizeIncludeDirectiveDto[];
  macros: OrgizeMacroDefinitionDto[];
  targets: OrgizeTargetDefinitionDto[];
  footnotes: OrgizeFootnoteEntryDto[];
}

export interface OrgizeOrgElementsSourceRangeDto extends OrgizeSourceRangeDto {
  raw: string;
}

export interface OrgizeOrgElementsPropertyDto {
  source: OrgizeOrgElementsSourceRangeDto;
  key: string;
  value: string;
}

export type OrgizeOrgElementsNodeDto = Record<string, unknown> & {
  source: OrgizeOrgElementsSourceRangeDto;
  kind: string;
};

export interface OrgizeOrgElementsIndexNodeDto {
  ordinal: number;
  category: string;
  kind: string;
  source: OrgizeOrgElementsSourceRangeDto;
  outlinePath: string[];
  context: string;
  summary: Record<string, unknown>;
}

export interface OrgizeOrgElementsIndexQueryDto {
  category?: string;
  kind?: string;
  context?: string;
  outlinePathPrefix?: string[];
  limit?: number;
}

export interface OrgizeOrgElementsSectionDto {
  source: OrgizeOrgElementsSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  tags: string[];
  effectiveTags: string[];
  anchor?: string | null;
  isComment: boolean;
  properties: OrgizeOrgElementsPropertyDto[];
  effectiveProperties: OrgizeOrgElementsPropertyDto[];
  elements: OrgizeOrgElementsNodeDto[];
  children: OrgizeOrgElementsSectionDto[];
}

export interface OrgizeOrgElementsDto {
  schemaVersion: 1;
  source: OrgizeOrgElementsSourceRangeDto;
  metadata: OrgizeKeywordDto[];
  filetags: string[];
  tagDefinitions: OrgizeTagDefinitionDto[];
  targets: OrgizeTargetDefinitionDto[];
  footnotes: OrgizeFootnoteEntryDto[];
  elements: OrgizeOrgElementsNodeDto[];
  sections: OrgizeOrgElementsSectionDto[];
  index: OrgizeOrgElementsIndexNodeDto[];
  sourceBlocks: OrgizeSourceBlockRecordDto[];
}

export type OrgizeOrgElementsIndexDto = OrgizeOrgElementsIndexNodeDto[];

export interface OrgizeTimestampMomentDto {
  year: number;
  month: number;
  day: number;
  dayName?: string | null;
  hour?: number | null;
  minute?: number | null;
}

export interface OrgizeTimestampDto {
  kind: "active" | "inactive" | "diary";
  raw: string;
  isRange: boolean;
  start?: OrgizeTimestampMomentDto | null;
  end?: OrgizeTimestampMomentDto | null;
}

export interface OrgizePlanningDto {
  deadline?: OrgizeTimestampDto | null;
  scheduled?: OrgizeTimestampDto | null;
  closed?: OrgizeTimestampDto | null;
}

export type OrgizeLinkSearchKindDto =
  | "headline"
  | "lineNumber"
  | "customId"
  | "regexp"
  | "text";

export interface OrgizeLinkSearchDto {
  raw: string;
  kind: OrgizeLinkSearchKindDto;
  normalized: string;
}

export interface OrgizeAttachmentLinkSearchDto {
  raw: string;
  kind: OrgizeLinkSearchKindDto;
}

export interface OrgizeAttachmentLinkDto {
  path: string;
  search?: OrgizeAttachmentLinkSearchDto | null;
}

export interface OrgizeFileLinkDto {
  protocol: string;
  path: string;
  pathKind: "empty" | "absolute" | "homeRelative" | "relative" | "remote";
  search?: OrgizeLinkSearchDto | null;
}

export interface OrgizeLinkDto {
  source: OrgizeSourceRangeDto;
  path: string;
  description: string;
  search?: OrgizeLinkSearchDto | null;
  attachment?: OrgizeAttachmentLinkDto | null;
  file?: OrgizeFileLinkDto | null;
}

export interface OrgizeTargetDto {
  source: OrgizeSourceRangeDto;
  kind: OrgizeTargetKindDto;
  key: string;
  value: string;
}

export interface OrgizeTextSliceDto {
  source: OrgizeSourceRangeDto;
  text: string;
}

export interface OrgizeLifecycleRecordDto {
  source: OrgizeSourceRangeDto;
  kind: string;
  raw: string;
}

export interface OrgizeSectionIndexRecordDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  body: OrgizeTextSliceDto[];
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  priority: OrgizePriorityDto;
  category?: string | null;
  tags: string[];
  effectiveTags: string[];
  properties: OrgizePropertyDto[];
  effectiveProperties: OrgizePropertyDto[];
  specialProperties: OrgizePropertyDto[];
  planning: OrgizePlanningDto;
  isComment: boolean;
  archive: OrgizeArchiveDto;
  attachment: OrgizeAttachmentStateDto;
  links: OrgizeLinkDto[];
  targets: OrgizeTargetDto[];
  lifecycle: OrgizeLifecycleRecordDto[];
}

export interface OrgizeSectionIndexResponseDto {
  schemaVersion: 1;
  records: OrgizeSectionIndexRecordDto[];
}

export type OrgizeSparseTreeMatchKindDto =
  | "query"
  | "title"
  | "body"
  | "tag"
  | "property"
  | "specialProperty"
  | "planning"
  | "priority"
  | "link"
  | "target";

export interface OrgizeSparseTreeMatchDto {
  source: OrgizeSourceRangeDto;
  kind: OrgizeSparseTreeMatchKindDto;
  key?: string | null;
  value: string;
}

export type OrgizeSparseTreeReceiptKindDto =
  | "candidate"
  | "visibilityFilterPassed"
  | "matchExpressionMatched"
  | "textMatched"
  | "defaultAllMatched"
  | "accepted"
  | "skippedComment"
  | "skippedArchived"
  | "skippedDone"
  | "skippedMatchExpression"
  | "skippedText";

export interface OrgizeSparseTreeReceiptDto {
  kind: OrgizeSparseTreeReceiptKindDto;
  message: string;
}

export interface OrgizeSparseTreeCardDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  matches: OrgizeSparseTreeMatchDto[];
  receipts: OrgizeSparseTreeReceiptDto[];
  preview?: string | null;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  priority: OrgizePriorityDto;
  category?: string | null;
  tags: string[];
  effectiveTags: string[];
  properties: OrgizePropertyDto[];
  specialProperties: OrgizePropertyDto[];
  planning: OrgizePlanningDto;
  archive: OrgizeArchiveDto;
  attachment: OrgizeAttachmentStateDto;
  links: OrgizeLinkDto[];
  targets: OrgizeTargetDto[];
  lifecycle: OrgizeLifecycleRecordDto[];
}

export type OrgizeSparseTreeSkipReasonDto =
  | "comment"
  | "archived"
  | "done"
  | "matchExpression"
  | "text";

export interface OrgizeSparseTreeSkipDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  reason: OrgizeSparseTreeSkipReasonDto;
  receipts: OrgizeSparseTreeReceiptDto[];
}

export interface OrgizeSparseTreeResponseDto {
  schemaVersion: 1;
  totalCandidates: number;
  cards: OrgizeSparseTreeCardDto[];
  skipped: OrgizeSparseTreeSkipDto[];
}

export interface OrgizeAgendaViewDateRequestDto {
  year: number;
  month: number;
  day: number;
}

export interface OrgizeAgendaViewJsonRequestDto {
  start: OrgizeAgendaViewDateRequestDto;
  end: OrgizeAgendaViewDateRequestDto;
  limit?: number | null;
  sortStrategy?: OrgizeAgendaViewSortSpecDto[] | null;
}

export interface OrgizeAgendaViewSortSpecDto {
  key:
    | "displayDate"
    | "time"
    | "kind"
    | "level"
    | "title"
    | "targetDate"
    | "scheduledDate"
    | "deadlineDate"
    | "priority"
    | "category"
    | "todoState";
  direction: "up" | "down" | "keep";
}

export interface OrgizeAgendaBlockSectionRequestDto {
  name: string;
  query: OrgizeAgendaViewJsonRequestDto;
}

export interface OrgizeAgendaBlockJsonRequestDto {
  title?: string | null;
  sections: OrgizeAgendaBlockSectionRequestDto[];
}

export interface OrgizeAgendaViewSortValueDto {
  key: string;
  value: string;
}

export interface OrgizeAgendaViewReceiptDto {
  kind: string;
  message: string;
}

export interface OrgizeAgendaUrgencyIngredientDto {
  kind:
    | "priority"
    | "deadline"
    | "scheduled"
    | "todoState"
    | "timeOfDay"
    | "tags"
    | "category"
    | "occurrence";
  score: number;
  message: string;
}

export interface OrgizeAgendaUrgencyScoreDto {
  total: number;
  ingredients: OrgizeAgendaUrgencyIngredientDto[];
}

export interface OrgizeAgendaViewCardDto {
  source: OrgizeSourceRangeDto;
  sortedPosition: number;
  kind: string;
  displayDate: string;
  targetDate: string;
  targetEndDate?: string | null;
  time?: string | null;
  endTime?: string | null;
  title: string;
  category?: string | null;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  effectiveTags: string[];
  blockers: OrgizeTaskBlockerRecordDto[];
  urgency: OrgizeAgendaUrgencyScoreDto;
  sortKeys: OrgizeAgendaViewSortValueDto[];
  receipts: OrgizeAgendaViewReceiptDto[];
}

export interface OrgizeAgendaViewSkipDto {
  source: OrgizeSourceRangeDto;
  sortedPosition: number;
  title: string;
  reason: string;
  limit?: number | null;
  blockers: OrgizeTaskBlockerRecordDto[];
  urgency: OrgizeAgendaUrgencyScoreDto;
  sortKeys: OrgizeAgendaViewSortValueDto[];
  receipts: OrgizeAgendaViewReceiptDto[];
}

export interface OrgizeAgendaViewResponseDto {
  schemaVersion: 1;
  totalCandidates: number;
  limit?: number | null;
  sortStrategy: OrgizeAgendaViewSortSpecDto[];
  cards: OrgizeAgendaViewCardDto[];
  skipped: OrgizeAgendaViewSkipDto[];
}

export interface OrgizeAgendaBlockSectionPlanDto {
  index: number;
  name: string;
  plan: OrgizeAgendaViewResponseDto;
}

export interface OrgizeAgendaBlockViewResponseDto {
  schemaVersion: 1;
  title: string;
  totalCandidates: number;
  sections: OrgizeAgendaBlockSectionPlanDto[];
}

export interface OrgizeViewPropertyDto {
  key: string;
  value: string;
}

export interface OrgizeViewIndexRecordDto {
  rangeStart: number;
  outline: string;
  level: number;
  title: string;
  bodyPreview: string;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  effectiveTags: string[];
  properties: OrgizeViewPropertyDto[];
  planning: {
    deadline?: string | null;
    scheduled?: string | null;
    closed?: string | null;
  };
}

export interface OrgizeViewIndexResponseDto {
  schemaVersion: 1;
  records: OrgizeViewIndexRecordDto[];
}

export interface OrgizeAttachmentRecordDto {
  source: OrgizeSourceRangeDto;
  sectionTitle: string;
  outlinePath: string[];
  directory?: OrgizeAttachmentDirectoryDto | null;
  links: OrgizeAttachmentLinkDto[];
}

export interface OrgizeAttachmentsResponseDto {
  schemaVersion: 1;
  records: OrgizeAttachmentRecordDto[];
}

export type OrgizeSourceBlockKindDto = "block" | "inlineSource";

export type OrgizeSourceBlockHeaderArgKindDto =
  | "cache"
  | "dir"
  | "eval"
  | "exports"
  | "hlines"
  | "noweb"
  | "results"
  | "session"
  | "tangle"
  | "var"
  | "other";

export interface OrgizeSourceBlockHeaderVarDto {
  name: string;
  assignment?: string | null;
}

export interface OrgizeSourceBlockHeaderArgDto {
  key: string;
  value?: string | null;
  raw: string;
  kind: OrgizeSourceBlockHeaderArgKindDto;
  source: "explicit" | "default";
  tokens: string[];
  variable?: OrgizeSourceBlockHeaderVarDto | null;
}

export interface OrgizeSourceBlockCodeRefDto {
  line: number;
  column: number;
  endColumn: number;
  name: string;
  raw: string;
}

export interface OrgizeSourceBlockTangleDto {
  raw: string;
  mode: "yes" | "no" | "file";
  target?: string | null;
}

export interface OrgizeSourceBlockResultDto {
  source: OrgizeSourceRangeDto;
  kind: "keyword" | "inlineMacro";
  hash?: string | null;
  name?: string | null;
  keywordValue: string;
  value: string;
}

export interface OrgizeSourceBlockRecordDto {
  source: OrgizeSourceRangeDto;
  kind: OrgizeSourceBlockKindDto;
  name?: string | null;
  language?: string | null;
  parameters?: string | null;
  headerArgs: OrgizeSourceBlockHeaderArgDto[];
  codeRefs: OrgizeSourceBlockCodeRefDto[];
  tangle?: OrgizeSourceBlockTangleDto | null;
  result?: OrgizeSourceBlockResultDto | null;
  value: string;
}

export interface OrgizeSourceBlocksResponseDto {
  schemaVersion: 1;
  records: OrgizeSourceBlockRecordDto[];
}

export interface OrgizeColumnViewScopeDto {
  kind: "documentKeyword" | "documentProperty" | "sectionProperty";
  level?: number | null;
  title?: string | null;
  outlinePath: string[];
}

export interface OrgizeColumnViewColumnDto {
  property: string;
  title?: string | null;
  width?: number | null;
  summaryOperator?: string | null;
  summaryFormat?: string | null;
  raw: string;
}

export interface OrgizeColumnViewRecordDto {
  source: OrgizeSourceRangeDto;
  scope: OrgizeColumnViewScopeDto;
  raw: string;
  columns: OrgizeColumnViewColumnDto[];
}

export interface OrgizeColumnViewsResponseDto {
  schemaVersion: 1;
  records: OrgizeColumnViewRecordDto[];
}

export interface OrgizeDynamicBlockParameterDto {
  key: string;
  value?: string | null;
  raw: string;
}

export type OrgizeDynamicBlockWriterDto =
  | "clocktable"
  | "columnview"
  | "unknown";

export type OrgizeDynamicBlockContentStateDto =
  | "empty"
  | "existingOutput";

export interface OrgizeDynamicBlockRecordDto {
  source: OrgizeSourceRangeDto;
  name: string;
  writer: OrgizeDynamicBlockWriterDto;
  parameters: OrgizeDynamicBlockParameterDto[];
  contentState: OrgizeDynamicBlockContentStateDto;
  contentLineCount: number;
}

export interface OrgizeDynamicBlocksResponseDto {
  schemaVersion: 1;
  records: OrgizeDynamicBlockRecordDto[];
}

export type OrgizePropertyInheritanceDto =
  | "none"
  | "all"
  | "selective"
  | "pattern";

export type OrgizePropertyAllowedValueScopeKindDto =
  | "fixedGlobal"
  | "document"
  | "section";

export interface OrgizePropertyAllowedValueScopeDto {
  kind: OrgizePropertyAllowedValueScopeKindDto;
  outlinePath: string[];
  level?: number | null;
  title?: string | null;
}

export interface OrgizePropertyAllowedValueRecordDto {
  source?: OrgizeSourceRangeDto | null;
  scope: OrgizePropertyAllowedValueScopeDto;
  property: string;
  descriptorKey: string;
  values: string[];
}

export interface OrgizePropertyProfileDto {
  inheritance: OrgizePropertyInheritanceDto;
  inheritedKeys: string[];
  allowedValues: OrgizePropertyAllowedValueRecordDto[];
}

export interface OrgizePropertyProfileResponseDto {
  schemaVersion: 1;
  profile: OrgizePropertyProfileDto;
}

export type OrgizeAgentCaptureKindDto =
  | "idea"
  | "articleNote"
  | "task"
  | "decision"
  | "preference"
  | "correction"
  | "memoryCandidate"
  | "evidence"
  | "note";

export type OrgizeAgentCaptureTargetKindDto =
  | "inbox"
  | "datetree"
  | "outlinePath"
  | "currentSection";

export type OrgizeAgentCaptureInsertPositionDto =
  | "append"
  | "prepend"
  | "firstChild"
  | "lastChild";

export type OrgizeAgentCaptureSourceKindDto =
  | "conversation"
  | "url"
  | "file"
  | "selection"
  | "article"
  | "code"
  | "other";

export type OrgizeAgentCaptureMemoryPolicyDto =
  | "none"
  | "candidate"
  | "background"
  | "decision"
  | "transient"
  | "supersedes";

export type OrgizeAgentCaptureApplicationActionDto =
  | "insertOrgEntry"
  | "resolveRuntimeTarget";

export type OrgizeAgentCaptureApplicationPreconditionKindDto =
  | "userConfirmation"
  | "sourceFileResolution"
  | "writeLock"
  | "datetreeResolution"
  | "outlinePathResolution"
  | "currentSectionResolution";

export interface OrgizeAgentCaptureDateDto {
  year: number;
  month: number;
  day: number;
}

export interface OrgizeAgentCaptureTimestampDto
  extends OrgizeAgentCaptureDateDto {
  hour?: number | null;
  minute?: number | null;
}

export interface OrgizeAgentCaptureTargetRequestDto {
  kind: OrgizeAgentCaptureTargetKindDto;
  sourceFile?: string | null;
  outlinePath?: string[] | null;
  date?: OrgizeAgentCaptureDateDto | null;
  insertPosition?: OrgizeAgentCaptureInsertPositionDto | null;
}

export interface OrgizeAgentCaptureSourceDto {
  kind?: OrgizeAgentCaptureSourceKindDto | null;
  actor?: string | null;
  uri?: string | null;
  label?: string | null;
}

export interface OrgizeAgentCapturePropertyDto {
  key: string;
  value: string;
}

export interface OrgizeAgentCaptureLinkDto {
  url: string;
  label?: string | null;
}

export interface OrgizeAgentCaptureRequestDto {
  kind: OrgizeAgentCaptureKindDto;
  title: string;
  body?: string | null;
  target?: OrgizeAgentCaptureTargetRequestDto | null;
  source?: OrgizeAgentCaptureSourceDto | null;
  capturedAt?: OrgizeAgentCaptureTimestampDto | null;
  tags?: string[] | null;
  properties?: OrgizeAgentCapturePropertyDto[] | null;
  quote?: string | null;
  links?: OrgizeAgentCaptureLinkDto[] | null;
  memoryPolicy?: OrgizeAgentCaptureMemoryPolicyDto | null;
  requiresConfirmation?: boolean | null;
}

export interface OrgizeAgentCaptureTargetDto {
  kind: OrgizeAgentCaptureTargetKindDto;
  sourceFile?: string | null;
  outlinePath: string[];
  date?: OrgizeAgentCaptureDateDto | null;
  insertPosition: OrgizeAgentCaptureInsertPositionDto;
}

export interface OrgizeAgentCaptureReceiptDto {
  kind:
    | "nonMutating"
    | "nativeOrgEntry"
    | "agentInterpreted"
    | "sourceProvenance"
    | "memoryPolicy"
    | "requiresConfirmation"
    | "applicationPlan";
  message: string;
}

export interface OrgizeAgentCaptureWarningDto {
  kind:
    | "emptyTitle"
    | "emptyBody"
    | "sanitizedTag"
    | "sanitizedPropertyKey"
    | "runtimeOwnedTarget";
  message: string;
}

export interface OrgizeAgentCapturePlanDto {
  target: OrgizeAgentCaptureTargetDto;
  orgEntry: string;
  application: OrgizeAgentCaptureApplicationDto;
  receipts: OrgizeAgentCaptureReceiptDto[];
  warnings: OrgizeAgentCaptureWarningDto[];
  requiresConfirmation: boolean;
}

export interface OrgizeAgentCaptureApplicationDto {
  action: OrgizeAgentCaptureApplicationActionDto;
  target: OrgizeAgentCaptureTargetDto;
  preconditions: OrgizeAgentCaptureApplicationPreconditionDto[];
}

export interface OrgizeAgentCaptureApplicationPreconditionDto {
  kind: OrgizeAgentCaptureApplicationPreconditionKindDto;
  message: string;
}

export interface OrgizeAgentCapturePlanResponseDto {
  schemaVersion: 1;
  plan: OrgizeAgentCapturePlanDto;
}

export type OrgizeRefileOutlinePathModeDto =
  | "none"
  | "outline"
  | "file"
  | "fullFilePath"
  | "bufferName"
  | "title";

export type OrgizeRefileTargetSpecKindDto =
  | "all"
  | "tag"
  | "todo"
  | "level"
  | "maxLevel"
  | "regexp";

export interface OrgizeRefileTargetSpecDto {
  kind: OrgizeRefileTargetSpecKindDto;
  value?: string | null;
}

export interface OrgizeRefileTargetReceiptDto {
  spec: OrgizeRefileTargetSpecDto;
  message: string;
}

export interface OrgizeRefileTargetDto {
  sourceFile?: string | null;
  source: OrgizeSourceRangeDto;
  level: number;
  title: string;
  outlinePath: string[];
  display: string;
  receipts: OrgizeRefileTargetReceiptDto[];
}

export type OrgizeRefileWarningKindDto =
  | "unsupportedRegexp"
  | "duplicateDisplay"
  | "sourceNotFound"
  | "targetNotFound"
  | "ambiguousSource"
  | "ambiguousTarget"
  | "parentNotFound"
  | "ambiguousParent"
  | "sameSourceAndTarget"
  | "targetInsideSource"
  | "copyMayDuplicateId";

export interface OrgizeRefileWarningDto {
  kind: OrgizeRefileWarningKindDto;
  message: string;
}

export interface OrgizeRefileTargetsRequestDto {
  sourceFile?: string | null;
  outlinePathMode?: OrgizeRefileOutlinePathModeDto | null;
  specs?: OrgizeRefileTargetSpecDto[] | null;
}

export interface OrgizeRefileTargetIndexResponseDto {
  schemaVersion: 1;
  sourceFile?: string | null;
  outlinePathMode: OrgizeRefileOutlinePathModeDto;
  specs: OrgizeRefileTargetSpecDto[];
  targets: OrgizeRefileTargetDto[];
  warnings: OrgizeRefileWarningDto[];
}

export type OrgizeRefileActionDto = "move" | "copy" | "goto";

export type OrgizeRefileInsertPositionDto = "lastChild" | "firstChild";

export type OrgizeRefileParentCreationModeDto = "never" | "plan" | "confirm";

export interface OrgizeRefilePlanRequestDto {
  sourceFile?: string | null;
  sourceOutlinePath: string[];
  targetOutlinePath: string[];
  action?: OrgizeRefileActionDto | null;
  insertPosition?: OrgizeRefileInsertPositionDto | null;
  parentCreation?: OrgizeRefileParentCreationModeDto | "allow" | null;
}

export interface OrgizeRefilePlanSectionDto {
  sourceFile?: string | null;
  source: OrgizeSourceRangeDto;
  level: number;
  title: string;
  outlinePath: string[];
  localIds: string[];
}

export interface OrgizeRefilePlanReceiptDto {
  kind:
    | "sourceResolved"
    | "targetResolved"
    | "insertPositionResolved"
    | "parentCreationPlanned"
    | "parentCreationRequiresConfirmation"
    | "nonMutating";
  message: string;
}

export interface OrgizeRefileCreateParentNodeDto {
  title: string;
  level: number;
  outlinePath: string[];
  display: string;
}

export interface OrgizeRefileCreateParentPlanDto {
  sourceFile?: string | null;
  existingParent: OrgizeRefileTargetDto;
  targetOutlinePath: string[];
  nodes: OrgizeRefileCreateParentNodeDto[];
  requiresConfirmation: boolean;
}

export interface OrgizeRefilePlanDto {
  sourceFile?: string | null;
  action: OrgizeRefileActionDto;
  insertPosition: OrgizeRefileInsertPositionDto;
  parentCreation: OrgizeRefileParentCreationModeDto;
  source?: OrgizeRefilePlanSectionDto | null;
  target?: OrgizeRefileTargetDto | null;
  createdTarget?: OrgizeRefileCreateParentPlanDto | null;
  receipts: OrgizeRefilePlanReceiptDto[];
  warnings: OrgizeRefileWarningDto[];
}

export interface OrgizeRefilePlanResponseDto {
  schemaVersion: 1;
  plan: OrgizeRefilePlanDto;
}

export interface OrgizeOrgDurationDto {
  raw: string;
  totalSeconds: number;
  totalMinutes: number;
}

export interface OrgizeProgressTodoSummaryDto {
  total: number;
  done: number;
  open: number;
}

export interface OrgizeProgressCheckboxSummaryDto {
  total: number;
  checked: number;
  unchecked: number;
  partial: number;
}

export interface OrgizeProgressStatisticCookieDto {
  source: OrgizeSourceRangeDto;
  raw: string;
  kind: "fraction" | "percent" | "unknown";
  done?: number | null;
  total?: number | null;
  percent?: number | null;
}

export interface OrgizeProgressEffortSummaryDto {
  local?: OrgizeOrgDurationDto | null;
  subtreeTotalSeconds: number;
}

export interface OrgizeTaskDependencyRecordDto {
  source: OrgizeSourceRangeDto;
  kind: "openDescendantTodo" | "openCheckbox" | "orderedProperty";
  count: number;
  message: string;
}

export interface OrgizeTaskBlockerTaskDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
}

export interface OrgizeTaskBlockerParentDto {
  source: OrgizeSourceRangeDto;
  orderedPropertySource: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
}

export interface OrgizeTaskBlockerRecordDto {
  kind: "orderedPreviousSibling";
  blocked: OrgizeTaskBlockerTaskDto;
  blocker: OrgizeTaskBlockerTaskDto;
  parent: OrgizeTaskBlockerParentDto;
  message: string;
}

export interface OrgizeTaskBlockersResponseDto {
  schemaVersion: 1;
  records: OrgizeTaskBlockerRecordDto[];
}

export type OrgizeSddKindDto =
  | "system"
  | "capability"
  | "view"
  | "decision"
  | "audit"
  | string;

export type OrgizeSddStatusDto =
  | "draft"
  | "review"
  | "accepted"
  | "deprecated"
  | "superseded"
  | string;

export interface OrgizeSddParentRefDto {
  raw: string;
  targetId?: string | null;
  label?: string | null;
}

export interface OrgizeSddNodeRecordDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  kind: OrgizeSddKindDto;
  kindKnown: boolean;
  id?: string | null;
  parent?: OrgizeSddParentRefDto | null;
  capability?: string | null;
  viewpoint?: string | null;
  concern?: string | null;
  quality?: string | null;
  rationale?: string | null;
  slug?: string | null;
  status?: OrgizeSddStatusDto | null;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  tags: string[];
}

export interface OrgizeSddResponseDto {
  schemaVersion: 1;
  records: OrgizeSddNodeRecordDto[];
}

export interface OrgizeMemoryJsonRequestDto {
  includeComments?: boolean;
  includeClosed?: boolean;
  includeArchived?: boolean;
  requiredTags?: string[];
  excludedTags?: string[];
}

export type OrgizeMemoryRecordStateDto = "current" | "background" | "closed" | "archived";

export type OrgizeAgentMemorySeverityDto = "action" | "suppressed" | "info";

export interface OrgizeMemoryStatsDto {
  totalRecords: number;
  currentRecords: number;
  backgroundRecords: number;
  closedRecords: number;
  archivedRecords: number;
  cards: number;
  actionCards: number;
  suppressedCards: number;
  infoCards: number;
  evidence: number;
  properties: number;
  links: number;
  authorityReasons: number;
}

export interface OrgizeMemoryPropertyDto {
  source: OrgizeSourceRangeDto;
  key: string;
  value: string;
}

export interface OrgizeMemoryEvidenceKindDto {
  code: string;
  label: string;
  family: string;
  detail?: string | null;
}

export interface OrgizeMemoryEvidenceDto {
  source: OrgizeSourceRangeDto;
  kind: OrgizeMemoryEvidenceKindDto;
  value: string;
}

export interface OrgizeMemoryLinkDto {
  source: OrgizeSourceRangeDto;
  path: string;
  description: string;
}

export interface OrgizeMemoryAuthorityReasonDto {
  kind: string;
  label: string;
  message: string;
}

export interface OrgizeAgentMemoryDecisionDto {
  code: string;
  kind: OrgizeMemoryRecordStateDto;
  severity: OrgizeAgentMemorySeverityDto;
  title: string;
  nextAction: string;
}

export interface OrgizeMemoryRecordDto {
  source: OrgizeSourceRangeDto;
  state: OrgizeMemoryRecordStateDto;
  level: number;
  title: string;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  tags: string[];
  effectiveTags: string[];
  anchor?: string | null;
  properties: OrgizeMemoryPropertyDto[];
  evidence: OrgizeMemoryEvidenceDto[];
  links: OrgizeMemoryLinkDto[];
}

export interface OrgizeAgentMemoryCardDto {
  source: OrgizeSourceRangeDto;
  decision: OrgizeAgentMemoryDecisionDto;
  authority: OrgizeMemoryAuthorityReasonDto[];
  title: string;
  todo?: string | null;
  todoState?: "todo" | "done" | null;
  tags: string[];
  effectiveTags: string[];
  anchor?: string | null;
  evidence: OrgizeMemoryEvidenceDto[];
  links: OrgizeMemoryLinkDto[];
}

export interface OrgizeMemoryFacetDto {
  code: string;
  label: string;
  count: number;
}

export interface OrgizeMemoryResponseDto {
  schemaVersion: 1;
  stats: OrgizeMemoryStatsDto;
  records: OrgizeMemoryRecordDto[];
  cards: OrgizeAgentMemoryCardDto[];
  evidenceKinds: OrgizeMemoryFacetDto[];
  authorityKinds: OrgizeMemoryFacetDto[];
}

export interface OrgizeProgressStatsRecordDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  todo: "none" | "todo" | "done";
  descendantTodos: OrgizeProgressTodoSummaryDto;
  checkboxes: OrgizeProgressCheckboxSummaryDto;
  statisticCookies: OrgizeProgressStatisticCookieDto[];
  effort: OrgizeProgressEffortSummaryDto;
  dependencies: OrgizeTaskDependencyRecordDto[];
}

export interface OrgizeProgressStatsResponseDto {
  schemaVersion: 1;
  records: OrgizeProgressStatsRecordDto[];
}

export interface OrgizeClockSummaryDto {
  entries: number;
  closedEntries: number;
  runningEntries: number;
  unparsedEntries: number;
  totalSeconds: number;
}

export type OrgizeClockEffortStatusDto =
  | "noEffort"
  | "underEffort"
  | "onEffort"
  | "overEffort";

export interface OrgizeClockEffortSummaryDto {
  local?: OrgizeOrgDurationDto | null;
  subtreeTotalSeconds: number;
  deltaSeconds: number;
  status: OrgizeClockEffortStatusDto;
}

export interface OrgizeClockRollupRecordDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  localClock: OrgizeClockSummaryDto;
  subtreeClock: OrgizeClockSummaryDto;
  effort: OrgizeClockEffortSummaryDto;
}

export interface OrgizeClockRollupResponseDto {
  schemaVersion: 1;
  records: OrgizeClockRollupRecordDto[];
}

export interface OrgizeClockIssueProfileRequestDto {
  maxDurationSeconds?: number | null;
  minDurationSeconds?: number | null;
  maxGapSeconds?: number | null;
  gapOkAroundMinutes?: number[];
}

export type OrgizeClockIssueKindDto =
  | "invalidClock"
  | "invalidDuration"
  | "invalidRange"
  | "noEndTime"
  | "longDuration"
  | "shortDuration"
  | "overlap"
  | "gap";

export interface OrgizeClockIssueClockDto {
  source: OrgizeSourceRangeDto;
  raw: string;
  start?: OrgizeClockTableTimeBoundDto | null;
  end?: OrgizeClockTableTimeBoundDto | null;
  durationSeconds?: number | null;
}

export interface OrgizeClockIssueFindingDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  kind: OrgizeClockIssueKindDto;
  message: string;
  clock: OrgizeClockIssueClockDto;
  previousClock?: OrgizeClockIssueClockDto | null;
  durationSeconds?: number | null;
  thresholdSeconds?: number | null;
}

export interface OrgizeClockIssuesResponseDto {
  schemaVersion: 1;
  findings: OrgizeClockIssueFindingDto[];
}

export interface OrgizeClockTableParameterDto {
  key: string;
  value?: string | null;
  raw: string;
}

export type OrgizeClockTableScopeKindDto =
  | "file"
  | "subtree"
  | "tree"
  | "treeLevel"
  | "agenda"
  | "agendaWithArchives"
  | "fileWithArchives"
  | "nil"
  | "external"
  | "unknown";

export interface OrgizeClockTableScopeDto {
  kind: OrgizeClockTableScopeKindDto;
  value?: string | null;
}

export interface OrgizeClockTableTimeBoundDto {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
}

export interface OrgizeClockTableTimeWindowDto {
  source: "block" | "tstartTend";
  start?: OrgizeClockTableTimeBoundDto | null;
  endExclusive?: OrgizeClockTableTimeBoundDto | null;
}

export interface OrgizeClockTableMatchFilterDto {
  expression: string;
}

export interface OrgizeClockTablePropertyColumnsDto {
  names: string[];
  inherit: boolean;
}

export interface OrgizeClockTablePropertyValueDto {
  name: string;
  value?: string | null;
  inherited: boolean;
}

export interface OrgizeClockTableRowDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  tableLevel: number;
  title: string;
  clock: OrgizeClockSummaryDto;
  effortTotalSeconds: number;
  effortDeltaSeconds: number;
  effortStatus: OrgizeClockEffortStatusDto;
  propertyValues: OrgizeClockTablePropertyValueDto[];
}

export type OrgizeClockTableWarningKindDto =
  | "unsupportedScope"
  | "timeRangePreserved"
  | "blockRangePreserved"
  | "matchPreserved"
  | "propertiesPreserved"
  | "stepPreserved";

export interface OrgizeClockTableWarningDto {
  kind: OrgizeClockTableWarningKindDto;
  message: string;
}

export interface OrgizeClockTablePlanDto {
  source: OrgizeSourceRangeDto;
  name: string;
  parameters: OrgizeClockTableParameterDto[];
  scope: OrgizeClockTableScopeDto;
  maxLevel: number;
  tstart?: string | null;
  tend?: string | null;
  timeWindow?: OrgizeClockTableTimeWindowDto | null;
  matchFilter?: OrgizeClockTableMatchFilterDto | null;
  propertyColumns?: OrgizeClockTablePropertyColumnsDto | null;
  rows: OrgizeClockTableRowDto[];
  warnings: OrgizeClockTableWarningDto[];
}

export interface OrgizeClockTablePlansResponseDto {
  schemaVersion: 1;
  plans: OrgizeClockTablePlanDto[];
}

export interface OrgizeLintFindingDto {
  code: string;
  severity: "error" | "warning";
  message: string;
  source: OrgizeSourceRangeDto;
}

export interface OrgizeLintResponseDto {
  schemaVersion: 1;
  findings: OrgizeLintFindingDto[];
}

export interface OrgizeSnapshotDto {
  schemaVersion: 1;
  metadata: OrgizeMetadataResponseDto;
  outline: OrgizeOutlineNodeDto[];
  sectionIndex: OrgizeSectionIndexRecordDto[];
  attachments: OrgizeAttachmentRecordDto[];
  sourceBlocks: OrgizeSourceBlockRecordDto[];
  columnViews: OrgizeColumnViewRecordDto[];
  dynamicBlocks: OrgizeDynamicBlockRecordDto[];
  propertyProfile: OrgizePropertyProfileDto;
  refileTargets: OrgizeRefileTargetDto[];
  includeExpansion: OrgizeIncludeExpansionEntryDto[];
  datetree: OrgizeDateTreeEntryDto[];
  progressStats: OrgizeProgressStatsRecordDto[];
  clockRollups: OrgizeClockRollupRecordDto[];
  clockTablePlans: OrgizeClockTablePlanDto[];
  clockIssues: OrgizeClockIssueFindingDto[];
  sdd: OrgizeSddNodeRecordDto[];
  memory: OrgizeMemoryResponseDto;
  lint: OrgizeLintFindingDto[];
}

export type OrgizeProjectionName =
  | "outline"
  | "metadata"
  | "orgElements"
  | "orgElementsIndex"
  | "lint"
  | "sectionIndex"
  | "sparseTree"
  | "agendaView"
  | "agendaBlock"
  | "viewIndex"
  | "attachments"
  | "sourceBlocks"
  | "columnViews"
  | "dynamicBlocks"
  | "propertyProfile"
  | "capturePlan"
  | "refileTargets"
  | "refilePlan"
  | "includeExpansion"
  | "datetree"
  | "progressStats"
  | "clockRollups"
  | "clockTablePlans"
  | "clockIssues"
  | "taskBlockers"
  | "sdd"
  | "memory"
  | "snapshot";

export type OrgizeProjectionDto =
  | OrgizeOutlineResponseDto
  | OrgizeMetadataResponseDto
  | OrgizeOrgElementsDto
  | OrgizeOrgElementsIndexDto
  | OrgizeLintResponseDto
  | OrgizeSectionIndexResponseDto
  | OrgizeSparseTreeResponseDto
  | OrgizeAgendaViewResponseDto
  | OrgizeAgendaBlockViewResponseDto
  | OrgizeViewIndexResponseDto
  | OrgizeAttachmentsResponseDto
  | OrgizeSourceBlocksResponseDto
  | OrgizeColumnViewsResponseDto
  | OrgizeDynamicBlocksResponseDto
  | OrgizePropertyProfileResponseDto
  | OrgizeAgentCapturePlanResponseDto
  | OrgizeRefileTargetIndexResponseDto
  | OrgizeRefilePlanResponseDto
  | OrgizeIncludeExpansionResponseDto
  | OrgizeDateTreeResponseDto
  | OrgizeProgressStatsResponseDto
  | OrgizeClockRollupResponseDto
  | OrgizeClockTablePlansResponseDto
  | OrgizeClockIssuesResponseDto
  | OrgizeTaskBlockersResponseDto
  | OrgizeSddResponseDto
  | OrgizeMemoryResponseDto
  | OrgizeSnapshotDto;
