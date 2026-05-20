declare namespace OrgizeDto {
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
}
