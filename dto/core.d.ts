declare namespace OrgizeDto {
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
}
