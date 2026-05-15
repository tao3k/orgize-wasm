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
  exportSettings: OrgizeExportSettingsDto;
  linkAbbreviations: OrgizeLinkAbbreviationDto[];
  includes: OrgizeIncludeDirectiveDto[];
  macros: OrgizeMacroDefinitionDto[];
  targets: OrgizeTargetDefinitionDto[];
  footnotes: OrgizeFootnoteEntryDto[];
}

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

export interface OrgizeSparseTreeCardDto {
  source: OrgizeSourceRangeDto;
  outlinePath: string[];
  level: number;
  title: string;
  matches: OrgizeSparseTreeMatchDto[];
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

export interface OrgizeSparseTreeResponseDto {
  schemaVersion: 1;
  cards: OrgizeSparseTreeCardDto[];
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
  includeExpansion: OrgizeIncludeExpansionEntryDto[];
  datetree: OrgizeDateTreeEntryDto[];
  lint: OrgizeLintFindingDto[];
}

export type OrgizeProjectionName =
  | "outline"
  | "metadata"
  | "lint"
  | "sectionIndex"
  | "sparseTree"
  | "viewIndex"
  | "attachments"
  | "sourceBlocks"
  | "columnViews"
  | "includeExpansion"
  | "datetree"
  | "snapshot";

export type OrgizeProjectionDto =
  | OrgizeOutlineResponseDto
  | OrgizeMetadataResponseDto
  | OrgizeLintResponseDto
  | OrgizeSectionIndexResponseDto
  | OrgizeSparseTreeResponseDto
  | OrgizeViewIndexResponseDto
  | OrgizeAttachmentsResponseDto
  | OrgizeSourceBlocksResponseDto
  | OrgizeColumnViewsResponseDto
  | OrgizeIncludeExpansionResponseDto
  | OrgizeDateTreeResponseDto
  | OrgizeSnapshotDto;
