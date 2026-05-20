declare namespace OrgizeDto {
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

  export type OrgizeSourceBlockReferenceKindDto =
    | "babelCall"
    | "headerVar"
    | "inlineCall"
    | "noweb";

  export interface OrgizeSourceBlockReferenceDto {
    source: OrgizeSourceRangeDto;
    kind: OrgizeSourceBlockReferenceKindDto;
    variable?: string | null;
    target: string;
    resolved: boolean;
  }

  export interface OrgizeSourceBlocksResponseDto {
    schemaVersion: 1;
    records: OrgizeSourceBlockRecordDto[];
    references: OrgizeSourceBlockReferenceDto[];
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

  export interface OrgizePropertyProfileDto {
    inheritance: OrgizePropertyInheritanceDto;
    inheritedKeys: string[];
    allowedValues: OrgizePropertyAllowedValueRecordDto[];
    schemaApplications: OrgizePropertySchemaApplicationDto[];
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
}
