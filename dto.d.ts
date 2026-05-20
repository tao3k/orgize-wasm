/// <reference path="./dto/core.d.ts" />
/// <reference path="./dto/elements.d.ts" />
/// <reference path="./dto/workspace.d.ts" />
/// <reference path="./dto/operations.d.ts" />
/// <reference path="./dto/clock.d.ts" />
/// <reference path="./dto/envelope.d.ts" />

export interface OrgizeSourcePositionDto extends OrgizeDto.OrgizeSourcePositionDto {}
export interface OrgizeSourceRangeDto extends OrgizeDto.OrgizeSourceRangeDto {}
export interface OrgizePriorityDto extends OrgizeDto.OrgizePriorityDto {}
export interface OrgizePriorityProfileDto extends OrgizeDto.OrgizePriorityProfileDto {}
export interface OrgizeArchiveDto extends OrgizeDto.OrgizeArchiveDto {}
export interface OrgizeAttachmentDirectorySourceDto extends OrgizeDto.OrgizeAttachmentDirectorySourceDto {}
export interface OrgizeAttachmentDirectoryDto extends OrgizeDto.OrgizeAttachmentDirectoryDto {}
export interface OrgizeAttachmentStateDto extends OrgizeDto.OrgizeAttachmentStateDto {}
export interface OrgizeOutlineNodeDto extends OrgizeDto.OrgizeOutlineNodeDto {}
export interface OrgizeOutlineResponseDto extends OrgizeDto.OrgizeOutlineResponseDto {}
export interface OrgizeKeywordAttributeDto extends OrgizeDto.OrgizeKeywordAttributeDto {}
export interface OrgizeKeywordDto extends OrgizeDto.OrgizeKeywordDto {}
export interface OrgizeExportSettingsDto extends OrgizeDto.OrgizeExportSettingsDto {}
export interface OrgizeTagDefinitionDto extends OrgizeDto.OrgizeTagDefinitionDto {}
export interface OrgizeTagDefinitionGroupDto extends OrgizeDto.OrgizeTagDefinitionGroupDto {}
export interface OrgizePropertyDto extends OrgizeDto.OrgizePropertyDto {}
export interface OrgizeLinkAbbreviationDto extends OrgizeDto.OrgizeLinkAbbreviationDto {}
export interface OrgizeIncludeOptionDto extends OrgizeDto.OrgizeIncludeOptionDto {}
export interface OrgizeIncludeDirectiveDto extends OrgizeDto.OrgizeIncludeDirectiveDto {}
export interface OrgizeIncludeLineSelectionDto extends OrgizeDto.OrgizeIncludeLineSelectionDto {}
export interface OrgizeIncludeExpansionModeDto extends OrgizeDto.OrgizeIncludeExpansionModeDto {}
export interface OrgizeIncludeExpansionEntryDto extends OrgizeDto.OrgizeIncludeExpansionEntryDto {}
export interface OrgizeIncludeExpansionResponseDto extends OrgizeDto.OrgizeIncludeExpansionResponseDto {}
export interface OrgizeDateTreeEntryDto extends OrgizeDto.OrgizeDateTreeEntryDto {}
export interface OrgizeDateTreeResponseDto extends OrgizeDto.OrgizeDateTreeResponseDto {}
export interface OrgizeMacroDefinitionDto extends OrgizeDto.OrgizeMacroDefinitionDto {}
export type OrgizeTargetKindDto = OrgizeDto.OrgizeTargetKindDto;
export interface OrgizeTargetDefinitionDto extends OrgizeDto.OrgizeTargetDefinitionDto {}
export interface OrgizeFootnoteEntryDto extends OrgizeDto.OrgizeFootnoteEntryDto {}
export interface OrgizeMetadataResponseDto extends OrgizeDto.OrgizeMetadataResponseDto {}
export interface OrgizeOrgElementsSourceRangeDto extends OrgizeDto.OrgizeOrgElementsSourceRangeDto {}
export interface OrgizeOrgElementsPropertyDto extends OrgizeDto.OrgizeOrgElementsPropertyDto {}
export type OrgizeOrgElementsNodeDto = OrgizeDto.OrgizeOrgElementsNodeDto;
export interface OrgizeOrgElementsIndexNodeDto extends OrgizeDto.OrgizeOrgElementsIndexNodeDto {}
export interface OrgizeOrgElementsIndexQueryDto extends OrgizeDto.OrgizeOrgElementsIndexQueryDto {}
export interface OrgizeOrgElementsSectionDto extends OrgizeDto.OrgizeOrgElementsSectionDto {}
export interface OrgizeOrgElementsDto extends OrgizeDto.OrgizeOrgElementsDto {}
export type OrgizeOrgElementsIndexDto = OrgizeDto.OrgizeOrgElementsIndexDto;
export interface OrgizeTimestampMomentDto extends OrgizeDto.OrgizeTimestampMomentDto {}
export interface OrgizeTimestampDto extends OrgizeDto.OrgizeTimestampDto {}
export interface OrgizePlanningDto extends OrgizeDto.OrgizePlanningDto {}
export type OrgizeLinkSearchKindDto = OrgizeDto.OrgizeLinkSearchKindDto;
export interface OrgizeLinkSearchDto extends OrgizeDto.OrgizeLinkSearchDto {}
export interface OrgizeAttachmentLinkSearchDto extends OrgizeDto.OrgizeAttachmentLinkSearchDto {}
export interface OrgizeAttachmentLinkDto extends OrgizeDto.OrgizeAttachmentLinkDto {}
export interface OrgizeFileLinkDto extends OrgizeDto.OrgizeFileLinkDto {}
export interface OrgizeLinkDto extends OrgizeDto.OrgizeLinkDto {}
export interface OrgizeTargetDto extends OrgizeDto.OrgizeTargetDto {}
export interface OrgizeTextSliceDto extends OrgizeDto.OrgizeTextSliceDto {}
export interface OrgizeLifecycleRecordDto extends OrgizeDto.OrgizeLifecycleRecordDto {}
export interface OrgizeSectionIndexRecordDto extends OrgizeDto.OrgizeSectionIndexRecordDto {}
export interface OrgizeSectionIndexResponseDto extends OrgizeDto.OrgizeSectionIndexResponseDto {}
export type OrgizeSparseTreeMatchKindDto = OrgizeDto.OrgizeSparseTreeMatchKindDto;
export interface OrgizeSparseTreeMatchDto extends OrgizeDto.OrgizeSparseTreeMatchDto {}
export type OrgizeSparseTreeReceiptKindDto = OrgizeDto.OrgizeSparseTreeReceiptKindDto;
export interface OrgizeSparseTreeReceiptDto extends OrgizeDto.OrgizeSparseTreeReceiptDto {}
export interface OrgizeSparseTreeCardDto extends OrgizeDto.OrgizeSparseTreeCardDto {}
export type OrgizeSparseTreeSkipReasonDto = OrgizeDto.OrgizeSparseTreeSkipReasonDto;
export interface OrgizeSparseTreeSkipDto extends OrgizeDto.OrgizeSparseTreeSkipDto {}
export interface OrgizeSparseTreeResponseDto extends OrgizeDto.OrgizeSparseTreeResponseDto {}
export interface OrgizeAgendaViewDateRequestDto extends OrgizeDto.OrgizeAgendaViewDateRequestDto {}
export interface OrgizeAgendaViewJsonRequestDto extends OrgizeDto.OrgizeAgendaViewJsonRequestDto {}
export interface OrgizeAgendaViewSortSpecDto extends OrgizeDto.OrgizeAgendaViewSortSpecDto {}
export interface OrgizeAgendaBlockSectionRequestDto extends OrgizeDto.OrgizeAgendaBlockSectionRequestDto {}
export interface OrgizeAgendaBlockJsonRequestDto extends OrgizeDto.OrgizeAgendaBlockJsonRequestDto {}
export interface OrgizeAgendaViewSortValueDto extends OrgizeDto.OrgizeAgendaViewSortValueDto {}
export interface OrgizeAgendaViewReceiptDto extends OrgizeDto.OrgizeAgendaViewReceiptDto {}
export interface OrgizeAgendaUrgencyIngredientDto extends OrgizeDto.OrgizeAgendaUrgencyIngredientDto {}
export interface OrgizeAgendaUrgencyScoreDto extends OrgizeDto.OrgizeAgendaUrgencyScoreDto {}
export interface OrgizeAgendaViewCardDto extends OrgizeDto.OrgizeAgendaViewCardDto {}
export interface OrgizeAgendaViewSkipDto extends OrgizeDto.OrgizeAgendaViewSkipDto {}
export interface OrgizeAgendaViewResponseDto extends OrgizeDto.OrgizeAgendaViewResponseDto {}
export interface OrgizeAgendaBlockSectionPlanDto extends OrgizeDto.OrgizeAgendaBlockSectionPlanDto {}
export interface OrgizeAgendaBlockViewResponseDto extends OrgizeDto.OrgizeAgendaBlockViewResponseDto {}
export interface OrgizeViewPropertyDto extends OrgizeDto.OrgizeViewPropertyDto {}
export interface OrgizeViewIndexRecordDto extends OrgizeDto.OrgizeViewIndexRecordDto {}
export interface OrgizeViewIndexResponseDto extends OrgizeDto.OrgizeViewIndexResponseDto {}
export interface OrgizeAttachmentRecordDto extends OrgizeDto.OrgizeAttachmentRecordDto {}
export interface OrgizeAttachmentsResponseDto extends OrgizeDto.OrgizeAttachmentsResponseDto {}
export type OrgizeSourceBlockKindDto = OrgizeDto.OrgizeSourceBlockKindDto;
export type OrgizeSourceBlockHeaderArgKindDto = OrgizeDto.OrgizeSourceBlockHeaderArgKindDto;
export interface OrgizeSourceBlockHeaderVarDto extends OrgizeDto.OrgizeSourceBlockHeaderVarDto {}
export interface OrgizeSourceBlockHeaderArgDto extends OrgizeDto.OrgizeSourceBlockHeaderArgDto {}
export interface OrgizeSourceBlockCodeRefDto extends OrgizeDto.OrgizeSourceBlockCodeRefDto {}
export interface OrgizeSourceBlockTangleDto extends OrgizeDto.OrgizeSourceBlockTangleDto {}
export interface OrgizeSourceBlockResultDto extends OrgizeDto.OrgizeSourceBlockResultDto {}
export interface OrgizeSourceBlockRecordDto extends OrgizeDto.OrgizeSourceBlockRecordDto {}
export type OrgizeSourceBlockReferenceKindDto = OrgizeDto.OrgizeSourceBlockReferenceKindDto;
export interface OrgizeSourceBlockReferenceDto extends OrgizeDto.OrgizeSourceBlockReferenceDto {}
export interface OrgizeSourceBlocksResponseDto extends OrgizeDto.OrgizeSourceBlocksResponseDto {}
export interface OrgizeColumnViewScopeDto extends OrgizeDto.OrgizeColumnViewScopeDto {}
export interface OrgizeColumnViewColumnDto extends OrgizeDto.OrgizeColumnViewColumnDto {}
export interface OrgizeColumnViewRecordDto extends OrgizeDto.OrgizeColumnViewRecordDto {}
export interface OrgizeColumnViewsResponseDto extends OrgizeDto.OrgizeColumnViewsResponseDto {}
export interface OrgizeDynamicBlockParameterDto extends OrgizeDto.OrgizeDynamicBlockParameterDto {}
export type OrgizeDynamicBlockWriterDto = OrgizeDto.OrgizeDynamicBlockWriterDto;
export type OrgizeDynamicBlockContentStateDto = OrgizeDto.OrgizeDynamicBlockContentStateDto;
export interface OrgizeDynamicBlockRecordDto extends OrgizeDto.OrgizeDynamicBlockRecordDto {}
export interface OrgizeDynamicBlocksResponseDto extends OrgizeDto.OrgizeDynamicBlocksResponseDto {}
export type OrgizePropertyInheritanceDto = OrgizeDto.OrgizePropertyInheritanceDto;
export type OrgizePropertyAllowedValueScopeKindDto = OrgizeDto.OrgizePropertyAllowedValueScopeKindDto;
export interface OrgizePropertyAllowedValueScopeDto extends OrgizeDto.OrgizePropertyAllowedValueScopeDto {}
export interface OrgizePropertyAllowedValueRecordDto extends OrgizeDto.OrgizePropertyAllowedValueRecordDto {}
export type OrgizePropertySchemaReferenceKindDto = OrgizeDto.OrgizePropertySchemaReferenceKindDto;
export type OrgizePropertySchemaScopeKindDto = OrgizeDto.OrgizePropertySchemaScopeKindDto;
export type OrgizePropertySchemaFindingKindDto = OrgizeDto.OrgizePropertySchemaFindingKindDto;
export type OrgizePropertySchemaValueRuleKindDto = OrgizeDto.OrgizePropertySchemaValueRuleKindDto;
export interface OrgizePropertySchemaReferenceDto extends OrgizeDto.OrgizePropertySchemaReferenceDto {}
export interface OrgizePropertySchemaScopeDto extends OrgizeDto.OrgizePropertySchemaScopeDto {}
export interface OrgizePropertySchemaFindingDto extends OrgizeDto.OrgizePropertySchemaFindingDto {}
export interface OrgizePropertySchemaApplicationDto extends OrgizeDto.OrgizePropertySchemaApplicationDto {}
export type OrgizePropertySchemaValueRuleDto = OrgizeDto.OrgizePropertySchemaValueRuleDto;
export interface OrgizePropertySchemaFieldDto extends OrgizeDto.OrgizePropertySchemaFieldDto {}
export interface OrgizePropertySchemaContractDto extends OrgizeDto.OrgizePropertySchemaContractDto {}
export interface OrgizePropertySchemaRegistryRequestDto extends OrgizeDto.OrgizePropertySchemaRegistryRequestDto {}
export interface OrgizePropertyProfileDto extends OrgizeDto.OrgizePropertyProfileDto {}
export interface OrgizePropertyProfileResponseDto extends OrgizeDto.OrgizePropertyProfileResponseDto {}
export type OrgizeAgentCaptureKindDto = OrgizeDto.OrgizeAgentCaptureKindDto;
export type OrgizeAgentCaptureTargetKindDto = OrgizeDto.OrgizeAgentCaptureTargetKindDto;
export type OrgizeAgentCaptureInsertPositionDto = OrgizeDto.OrgizeAgentCaptureInsertPositionDto;
export type OrgizeAgentCaptureSourceKindDto = OrgizeDto.OrgizeAgentCaptureSourceKindDto;
export type OrgizeAgentCaptureMemoryPolicyDto = OrgizeDto.OrgizeAgentCaptureMemoryPolicyDto;
export type OrgizeAgentCaptureApplicationActionDto = OrgizeDto.OrgizeAgentCaptureApplicationActionDto;
export type OrgizeAgentCaptureApplicationPreconditionKindDto = OrgizeDto.OrgizeAgentCaptureApplicationPreconditionKindDto;
export interface OrgizeAgentCaptureDateDto extends OrgizeDto.OrgizeAgentCaptureDateDto {}
export interface OrgizeAgentCaptureTimestampDto extends OrgizeDto.OrgizeAgentCaptureTimestampDto {}
export interface OrgizeAgentCaptureTargetRequestDto extends OrgizeDto.OrgizeAgentCaptureTargetRequestDto {}
export interface OrgizeAgentCaptureSourceDto extends OrgizeDto.OrgizeAgentCaptureSourceDto {}
export interface OrgizeAgentCapturePropertyDto extends OrgizeDto.OrgizeAgentCapturePropertyDto {}
export interface OrgizeAgentCaptureLinkDto extends OrgizeDto.OrgizeAgentCaptureLinkDto {}
export interface OrgizeAgentCaptureRequestDto extends OrgizeDto.OrgizeAgentCaptureRequestDto {}
export interface OrgizeAgentCaptureTargetDto extends OrgizeDto.OrgizeAgentCaptureTargetDto {}
export interface OrgizeAgentCaptureReceiptDto extends OrgizeDto.OrgizeAgentCaptureReceiptDto {}
export interface OrgizeAgentCaptureWarningDto extends OrgizeDto.OrgizeAgentCaptureWarningDto {}
export interface OrgizeAgentCapturePlanDto extends OrgizeDto.OrgizeAgentCapturePlanDto {}
export interface OrgizeAgentCaptureApplicationDto extends OrgizeDto.OrgizeAgentCaptureApplicationDto {}
export interface OrgizeAgentCaptureApplicationPreconditionDto extends OrgizeDto.OrgizeAgentCaptureApplicationPreconditionDto {}
export interface OrgizeAgentCapturePlanResponseDto extends OrgizeDto.OrgizeAgentCapturePlanResponseDto {}
export type OrgizeRefileOutlinePathModeDto = OrgizeDto.OrgizeRefileOutlinePathModeDto;
export type OrgizeRefileTargetSpecKindDto = OrgizeDto.OrgizeRefileTargetSpecKindDto;
export interface OrgizeRefileTargetSpecDto extends OrgizeDto.OrgizeRefileTargetSpecDto {}
export interface OrgizeRefileTargetReceiptDto extends OrgizeDto.OrgizeRefileTargetReceiptDto {}
export interface OrgizeRefileTargetDto extends OrgizeDto.OrgizeRefileTargetDto {}
export type OrgizeRefileWarningKindDto = OrgizeDto.OrgizeRefileWarningKindDto;
export interface OrgizeRefileWarningDto extends OrgizeDto.OrgizeRefileWarningDto {}
export interface OrgizeRefileTargetsRequestDto extends OrgizeDto.OrgizeRefileTargetsRequestDto {}
export interface OrgizeRefileTargetIndexResponseDto extends OrgizeDto.OrgizeRefileTargetIndexResponseDto {}
export type OrgizeRefileActionDto = OrgizeDto.OrgizeRefileActionDto;
export type OrgizeRefileInsertPositionDto = OrgizeDto.OrgizeRefileInsertPositionDto;
export type OrgizeRefileParentCreationModeDto = OrgizeDto.OrgizeRefileParentCreationModeDto;
export interface OrgizeRefilePlanRequestDto extends OrgizeDto.OrgizeRefilePlanRequestDto {}
export interface OrgizeRefilePlanSectionDto extends OrgizeDto.OrgizeRefilePlanSectionDto {}
export interface OrgizeRefilePlanReceiptDto extends OrgizeDto.OrgizeRefilePlanReceiptDto {}
export interface OrgizeRefileCreateParentNodeDto extends OrgizeDto.OrgizeRefileCreateParentNodeDto {}
export interface OrgizeRefileCreateParentPlanDto extends OrgizeDto.OrgizeRefileCreateParentPlanDto {}
export interface OrgizeRefilePlanDto extends OrgizeDto.OrgizeRefilePlanDto {}
export interface OrgizeRefilePlanResponseDto extends OrgizeDto.OrgizeRefilePlanResponseDto {}
export interface OrgizeOrgDurationDto extends OrgizeDto.OrgizeOrgDurationDto {}
export interface OrgizeProgressTodoSummaryDto extends OrgizeDto.OrgizeProgressTodoSummaryDto {}
export interface OrgizeProgressCheckboxSummaryDto extends OrgizeDto.OrgizeProgressCheckboxSummaryDto {}
export interface OrgizeProgressStatisticCookieDto extends OrgizeDto.OrgizeProgressStatisticCookieDto {}
export interface OrgizeProgressEffortSummaryDto extends OrgizeDto.OrgizeProgressEffortSummaryDto {}
export interface OrgizeTaskDependencyRecordDto extends OrgizeDto.OrgizeTaskDependencyRecordDto {}
export interface OrgizeTaskBlockerTaskDto extends OrgizeDto.OrgizeTaskBlockerTaskDto {}
export interface OrgizeTaskBlockerParentDto extends OrgizeDto.OrgizeTaskBlockerParentDto {}
export interface OrgizeTaskBlockerRecordDto extends OrgizeDto.OrgizeTaskBlockerRecordDto {}
export interface OrgizeTaskBlockersResponseDto extends OrgizeDto.OrgizeTaskBlockersResponseDto {}
export type OrgizeSddKindDto = OrgizeDto.OrgizeSddKindDto;
export type OrgizeSddStatusDto = OrgizeDto.OrgizeSddStatusDto;
export interface OrgizeSddParentRefDto extends OrgizeDto.OrgizeSddParentRefDto {}
export interface OrgizeSddNodeRecordDto extends OrgizeDto.OrgizeSddNodeRecordDto {}
export interface OrgizeSddResponseDto extends OrgizeDto.OrgizeSddResponseDto {}
export interface OrgizeMemoryJsonRequestDto extends OrgizeDto.OrgizeMemoryJsonRequestDto {}
export type OrgizeMemoryRecordStateDto = OrgizeDto.OrgizeMemoryRecordStateDto;
export type OrgizeAgentMemorySeverityDto = OrgizeDto.OrgizeAgentMemorySeverityDto;
export interface OrgizeMemoryStatsDto extends OrgizeDto.OrgizeMemoryStatsDto {}
export interface OrgizeMemoryPropertyDto extends OrgizeDto.OrgizeMemoryPropertyDto {}
export interface OrgizeMemoryEvidenceKindDto extends OrgizeDto.OrgizeMemoryEvidenceKindDto {}
export interface OrgizeMemoryEvidenceDto extends OrgizeDto.OrgizeMemoryEvidenceDto {}
export interface OrgizeMemoryLinkDto extends OrgizeDto.OrgizeMemoryLinkDto {}
export interface OrgizeMemoryAuthorityReasonDto extends OrgizeDto.OrgizeMemoryAuthorityReasonDto {}
export interface OrgizeAgentMemoryDecisionDto extends OrgizeDto.OrgizeAgentMemoryDecisionDto {}
export interface OrgizeMemoryRecordDto extends OrgizeDto.OrgizeMemoryRecordDto {}
export interface OrgizeAgentMemoryCardDto extends OrgizeDto.OrgizeAgentMemoryCardDto {}
export interface OrgizeMemoryFacetDto extends OrgizeDto.OrgizeMemoryFacetDto {}
export interface OrgizeMemoryResponseDto extends OrgizeDto.OrgizeMemoryResponseDto {}
export type OrgizeCryptWarningKindDto = OrgizeDto.OrgizeCryptWarningKindDto;
export interface OrgizeCryptKeyDto extends OrgizeDto.OrgizeCryptKeyDto {}
export interface OrgizeCryptWarningDto extends OrgizeDto.OrgizeCryptWarningDto {}
export interface OrgizeCryptRecordDto extends OrgizeDto.OrgizeCryptRecordDto {}
export interface OrgizeCryptResponseDto extends OrgizeDto.OrgizeCryptResponseDto {}
export type OrgizeRuntimeTimerContextDto = OrgizeDto.OrgizeRuntimeTimerContextDto;
export type OrgizeRuntimeBoundaryKindDto = OrgizeDto.OrgizeRuntimeBoundaryKindDto;
export type OrgizeRuntimeWarningKindDto = OrgizeDto.OrgizeRuntimeWarningKindDto;
export interface OrgizeFeedStatusRecordDto extends OrgizeDto.OrgizeFeedStatusRecordDto {}
export interface OrgizeTimerRecordDto extends OrgizeDto.OrgizeTimerRecordDto {}
export interface OrgizeMobileReadonlyKeywordDto extends OrgizeDto.OrgizeMobileReadonlyKeywordDto {}
export interface OrgizeMobilePriorityDeclarationDto extends OrgizeDto.OrgizeMobilePriorityDeclarationDto {}
export interface OrgizeMobileIndexLinkDto extends OrgizeDto.OrgizeMobileIndexLinkDto {}
export interface OrgizeMobilePropertyDto extends OrgizeDto.OrgizeMobilePropertyDto {}
export interface OrgizeMobileFlaggedSectionDto extends OrgizeDto.OrgizeMobileFlaggedSectionDto {}
export interface OrgizeMobileOriginalIdDto extends OrgizeDto.OrgizeMobileOriginalIdDto {}
export interface OrgizeMobileSyncMetadataDto extends OrgizeDto.OrgizeMobileSyncMetadataDto {}
export interface OrgizeRuntimeMetadataBoundaryDto extends OrgizeDto.OrgizeRuntimeMetadataBoundaryDto {}
export interface OrgizeRuntimeMetadataWarningDto extends OrgizeDto.OrgizeRuntimeMetadataWarningDto {}
export interface OrgizeRuntimeMetadataResponseDto extends OrgizeDto.OrgizeRuntimeMetadataResponseDto {}
export interface OrgizeProgressStatsRecordDto extends OrgizeDto.OrgizeProgressStatsRecordDto {}
export interface OrgizeProgressStatsResponseDto extends OrgizeDto.OrgizeProgressStatsResponseDto {}
export interface OrgizeClockSummaryDto extends OrgizeDto.OrgizeClockSummaryDto {}
export type OrgizeClockEffortStatusDto = OrgizeDto.OrgizeClockEffortStatusDto;
export interface OrgizeClockEffortSummaryDto extends OrgizeDto.OrgizeClockEffortSummaryDto {}
export interface OrgizeClockRollupRecordDto extends OrgizeDto.OrgizeClockRollupRecordDto {}
export interface OrgizeClockRollupResponseDto extends OrgizeDto.OrgizeClockRollupResponseDto {}
export interface OrgizeClockIssueProfileRequestDto extends OrgizeDto.OrgizeClockIssueProfileRequestDto {}
export type OrgizeClockIssueKindDto = OrgizeDto.OrgizeClockIssueKindDto;
export interface OrgizeClockIssueClockDto extends OrgizeDto.OrgizeClockIssueClockDto {}
export interface OrgizeClockIssueFindingDto extends OrgizeDto.OrgizeClockIssueFindingDto {}
export interface OrgizeClockIssuesResponseDto extends OrgizeDto.OrgizeClockIssuesResponseDto {}
export interface OrgizeClockTableParameterDto extends OrgizeDto.OrgizeClockTableParameterDto {}
export type OrgizeClockTableScopeKindDto = OrgizeDto.OrgizeClockTableScopeKindDto;
export interface OrgizeClockTableScopeDto extends OrgizeDto.OrgizeClockTableScopeDto {}
export interface OrgizeClockTableTimeBoundDto extends OrgizeDto.OrgizeClockTableTimeBoundDto {}
export interface OrgizeClockTableTimeWindowDto extends OrgizeDto.OrgizeClockTableTimeWindowDto {}
export interface OrgizeClockTableMatchFilterDto extends OrgizeDto.OrgizeClockTableMatchFilterDto {}
export interface OrgizeClockTablePropertyColumnsDto extends OrgizeDto.OrgizeClockTablePropertyColumnsDto {}
export interface OrgizeClockTablePropertyValueDto extends OrgizeDto.OrgizeClockTablePropertyValueDto {}
export interface OrgizeClockTableRowDto extends OrgizeDto.OrgizeClockTableRowDto {}
export type OrgizeClockTableWarningKindDto = OrgizeDto.OrgizeClockTableWarningKindDto;
export interface OrgizeClockTableWarningDto extends OrgizeDto.OrgizeClockTableWarningDto {}
export interface OrgizeClockTablePlanDto extends OrgizeDto.OrgizeClockTablePlanDto {}
export interface OrgizeClockTablePlansResponseDto extends OrgizeDto.OrgizeClockTablePlansResponseDto {}
export interface OrgizeLintFindingDto extends OrgizeDto.OrgizeLintFindingDto {}
export interface OrgizeLintResponseDto extends OrgizeDto.OrgizeLintResponseDto {}
export interface OrgizeSnapshotDto extends OrgizeDto.OrgizeSnapshotDto {}
export type OrgizeProjectionName = OrgizeDto.OrgizeProjectionName;
export type OrgizeProjectionDto = OrgizeDto.OrgizeProjectionDto;
