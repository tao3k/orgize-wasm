declare namespace OrgizeDto {
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

  export type OrgizeCryptWarningKindDto =
    | "inheritedCryptTag"
    | "plaintextCryptBody"
    | "cryptKeyWithoutCryptTag";

  export interface OrgizeCryptKeyDto {
    source: OrgizeSourceRangeDto;
    value: string;
    inherited: boolean;
  }

  export interface OrgizeCryptWarningDto {
    kind: OrgizeCryptWarningKindDto;
    message: string;
  }

  export interface OrgizeCryptRecordDto {
    source: OrgizeSourceRangeDto;
    outlinePath: string[];
    level: number;
    title: string;
    tag: string;
    hasDirectTag: boolean;
    hasInheritedTag: boolean;
    cryptKey?: OrgizeCryptKeyDto | null;
    encryptedPayload: boolean;
    bodyIsOpaque: boolean;
    warnings: OrgizeCryptWarningDto[];
  }

  export interface OrgizeCryptResponseDto {
    schemaVersion: 1;
    records: OrgizeCryptRecordDto[];
  }

  export type OrgizeRuntimeTimerContextDto =
    | "headline"
    | "paragraph"
    | "listItemTag";

  export type OrgizeRuntimeBoundaryKindDto =
    | "feedNetworkUpdate"
    | "timerRuntimeState"
    | "mobileFilesystemSync"
    | "orgPersistCache";

  export type OrgizeRuntimeWarningKindDto =
    | "unreadableFeedStatus"
    | "mobileReadonlyWithoutIndexLinks";

  export interface OrgizeFeedStatusRecordDto {
    source: OrgizeSourceRangeDto;
    sectionTitle: string;
    drawer: string;
    raw: string;
    entryCount: number;
    readable: boolean;
  }

  export interface OrgizeTimerRecordDto {
    source: OrgizeSourceRangeDto;
    outlinePath: string[];
    context: OrgizeRuntimeTimerContextDto;
    raw: string;
    totalSeconds: number;
  }

  export interface OrgizeMobileReadonlyKeywordDto {
    source: OrgizeSourceRangeDto;
    value: string;
  }

  export interface OrgizeMobilePriorityDeclarationDto {
    source: OrgizeSourceRangeDto;
    values: string[];
    raw: string;
  }

  export interface OrgizeMobileIndexLinkDto {
    source: OrgizeSourceRangeDto;
    title: string;
    file: string;
    description: string;
  }

  export interface OrgizeMobilePropertyDto {
    source: OrgizeSourceRangeDto;
    key: string;
    value: string;
  }

  export interface OrgizeMobileFlaggedSectionDto {
    source: OrgizeSourceRangeDto;
    outlinePath: string[];
    title: string;
    originalId?: string | null;
    mobileProperties: OrgizeMobilePropertyDto[];
  }

  export interface OrgizeMobileOriginalIdDto {
    source: OrgizeSourceRangeDto;
    outlinePath: string[];
    title: string;
    value: string;
  }

  export interface OrgizeMobileSyncMetadataDto {
    readonly: OrgizeMobileReadonlyKeywordDto[];
    allPriorities: OrgizeMobilePriorityDeclarationDto[];
    indexLinks: OrgizeMobileIndexLinkDto[];
    flaggedSections: OrgizeMobileFlaggedSectionDto[];
    originalIds: OrgizeMobileOriginalIdDto[];
  }

  export interface OrgizeRuntimeMetadataBoundaryDto {
    kind: OrgizeRuntimeBoundaryKindDto;
    message: string;
  }

  export interface OrgizeRuntimeMetadataWarningDto {
    kind: OrgizeRuntimeWarningKindDto;
    message: string;
  }

  export interface OrgizeRuntimeMetadataResponseDto {
    schemaVersion: 1;
    feeds: OrgizeFeedStatusRecordDto[];
    timers: OrgizeTimerRecordDto[];
    mobile: OrgizeMobileSyncMetadataDto;
    boundaries: OrgizeRuntimeMetadataBoundaryDto[];
    warnings: OrgizeRuntimeMetadataWarningDto[];
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
}
