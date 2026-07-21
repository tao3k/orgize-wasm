declare namespace OrgizeDto {
  export interface OrgizeInteractiveCategoryDto {
    key: string;
    value: string;
    detail: boolean;
  }

  export interface OrgizeInteractiveChoiceEntryDto {
    number: string;
    id: string;
    contract?: string | null;
    full: string;
    useIf: string;
  }

  export interface OrgizeInteractiveChoiceDto {
    source: OrgizeSourceRangeDto;
    id: string;
    method: "choice";
    stage: string;
    group?: string | null;
    target?: string | null;
    create?: string | null;
    info: string;
    categories: OrgizeInteractiveCategoryDto[];
    entries: OrgizeInteractiveChoiceEntryDto[];
  }

  export interface OrgizeInteractiveResponseDto {
    schemaVersion: 1;
    choices: OrgizeInteractiveChoiceDto[];
  }
}
