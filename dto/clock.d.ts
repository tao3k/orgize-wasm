declare namespace OrgizeDto {
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
}
