declare namespace OrgizeDto {
  export type OrgizeAttachmentArchiveDeletePolicyDto =
    | "notConfigured"
    | "never"
    | "query"
    | "always";

  export interface OrgizeAttachmentInventoryRequestDto {
    baseDir?: string;
    attachIdDir?: string;
    checkVcs?: boolean;
    checkAnnex?: boolean;
    archiveDeletePolicy?: OrgizeAttachmentArchiveDeletePolicyDto;
    scanOrphans?: boolean;
  }

  export interface OrgizeAttachmentInventoryEntryKindDto {
    label: "directory" | "link";
    directorySource?: OrgizeAttachmentDirectorySourceDto | null;
    link?: OrgizeAttachmentLinkDto | null;
  }

  export interface OrgizeAttachmentAnnexEvidenceDto {
    status:
      | "notChecked"
      | "notAnnexRepository"
      | "gitAnnexUnavailable"
      | "present"
      | "missing"
      | "unknown";
    raw?: string | null;
  }

  export interface OrgizeAttachmentVcsEvidenceDto {
    status:
      | "notChecked"
      | "clean"
      | "modified"
      | "untracked"
      | "missing"
      | "notInGitWorktree"
      | "gitUnavailable";
    annex: OrgizeAttachmentAnnexEvidenceDto;
    raw?: string | null;
  }

  export interface OrgizeAttachmentInventoryEntryDto {
    source: OrgizeSourceRangeDto;
    sectionTitle: string;
    kind: OrgizeAttachmentInventoryEntryKindDto;
    path: string;
    absolutePath: string;
    exists: boolean;
    vcs: OrgizeAttachmentVcsEvidenceDto;
  }

  export type OrgizeAttachmentDisplayMediaKindDto =
    | "image"
    | "video"
    | "audio"
    | "pdf"
    | "other";

  export interface OrgizeAttachmentDisplayRecordDto {
    source: OrgizeSourceRangeDto;
    sectionTitle: string;
    sectionTitleText: string;
    outlinePath: string[];
    outlinePathText: string[];
    tags: string[];
    effectiveTags: string[];
    attachmentId?: string | null;
    directoryPath: string;
    linkPath: string;
    absolutePath: string;
    exists: boolean;
    mediaKind: OrgizeAttachmentDisplayMediaKindDto;
  }

  export type OrgizeAttachmentSyncActionKindDto =
    | "missingDirectory"
    | "missingLinkedFile"
    | "orphanFile"
    | "emptyDirectory"
    | "staleAttachTag";

  export interface OrgizeAttachmentSyncActionDto {
    kind: OrgizeAttachmentSyncActionKindDto;
    source: OrgizeSourceRangeDto;
    sectionTitle: string;
    path: string;
    absolutePath?: string | null;
    message: string;
  }

  export interface OrgizeAttachmentSyncPlanDto {
    actions: OrgizeAttachmentSyncActionDto[];
  }

  export interface OrgizeAttachmentArchiveAdviceDto {
    source: OrgizeSourceRangeDto;
    sectionTitle: string;
    policy: OrgizeAttachmentArchiveDeletePolicyDto;
    path: string;
    message: string;
  }

  export interface OrgizeAttachmentInventoryWarningDto {
    kind: "missingDirectory" | "missingPath";
    message: string;
  }

  export interface OrgizeAttachmentInventoryResponseDto {
    schemaVersion: 1;
    entries: OrgizeAttachmentInventoryEntryDto[];
    display: OrgizeAttachmentDisplayRecordDto[];
    syncPlan: OrgizeAttachmentSyncPlanDto;
    archiveAdvice: OrgizeAttachmentArchiveAdviceDto[];
    warnings: OrgizeAttachmentInventoryWarningDto[];
  }
}
