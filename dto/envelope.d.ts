declare namespace OrgizeDto {
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
    crypt: OrgizeCryptRecordDto[];
    runtimeMetadata: OrgizeRuntimeMetadataResponseDto;
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
    | "crypt"
    | "runtimeMetadata"
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
    | OrgizeCryptResponseDto
    | OrgizeRuntimeMetadataResponseDto
    | OrgizeSnapshotDto;
}
