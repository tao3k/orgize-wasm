declare namespace OrgizeDto {
  export type OrgizeSourceBlockKindDto = "block" | "inlineSource";

  export type OrgizeSourceBlockHeaderArgKindDto =
    | "cache"
    | "dir"
    | "eval"
    | "exports"
    | "file"
    | "fileDesc"
    | "fileExt"
    | "fileMode"
    | "hlines"
    | "noweb"
    | "outputDir"
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
    mkdirp: OrgizeSourceBlockTangleMkdirpDto;
    comments: OrgizeSourceBlockTangleCommentsDto;
    shebang?: string | null;
    noweb: OrgizeSourceBlockTangleNowebDto;
  }

  export interface OrgizeSourceBlockTangleMkdirpDto {
    raw: string;
    enabled: boolean;
  }

  export interface OrgizeSourceBlockTangleCommentsDto {
    raw: string;
    mode: "no" | "link" | "yes" | "org" | "both" | "noweb" | "other";
  }

  export interface OrgizeSourceBlockTangleNowebDto {
    raw: string;
    mode: "disabled" | "expand" | "strip";
  }

  export interface OrgizeSourceBlockResultDto {
    source: OrgizeSourceRangeDto;
    kind: "keyword" | "inlineMacro";
    hash?: string | null;
    name?: string | null;
    keywordValue: string;
    value: string;
  }

  export interface OrgizeSourceBlockResultOptionsDto {
    raw: string;
    source: "explicit" | "default";
    tokens: string[];
    collection?: "file" | "list" | "vector" | "table" | "scalar" | "verbatim" | null;
    format?: "raw" | "html" | "latex" | "org" | "code" | "pp" | "drawer" | "link" | "graphics" | null;
    handling: "replace" | "silent" | "none" | "discard" | "append" | "prepend";
    valueType: "value" | "output";
    unknown: string[];
    file?: OrgizeSourceBlockResultFileDto | null;
  }

  export interface OrgizeSourceBlockResultFileDto {
    target: string;
    description?: string | null;
    extension?: string | null;
    fileMode?: string | null;
    outputDir?: string | null;
  }

  export interface OrgizeSourceBlockExecutionPlanDto {
    eval: OrgizeSourceBlockEvalDto;
    exports: OrgizeSourceBlockExportsDto;
    cache: OrgizeSourceBlockCacheDto;
    session: OrgizeSourceBlockSessionDto;
    directory?: OrgizeSourceBlockDirectoryDto | null;
    hlines: OrgizeSourceBlockBooleanHeaderDto;
    noweb: OrgizeSourceBlockNowebPlanDto;
  }

  export interface OrgizeSourceBlockEvalDto {
    raw: string;
    source: "explicit" | "default";
    policy:
      | "yes"
      | "no"
      | "noExport"
      | "stripExport"
      | "neverExport"
      | "eval"
      | "never"
      | "query"
      | "other";
  }

  export interface OrgizeSourceBlockExportsDto {
    raw: string;
    source: "explicit" | "default";
    policy: "code" | "results" | "both" | "none" | "other";
  }

  export interface OrgizeSourceBlockCacheDto {
    raw: string;
    source: "explicit" | "default";
    enabled: boolean;
  }

  export interface OrgizeSourceBlockSessionDto {
    raw: string;
    source: "explicit" | "default";
    name?: string | null;
    active: boolean;
  }

  export interface OrgizeSourceBlockDirectoryDto {
    raw: string;
    source: "explicit" | "default";
    target: string;
    kind: "path" | "attachment";
  }

  export interface OrgizeSourceBlockBooleanHeaderDto {
    raw: string;
    source: "explicit" | "default";
    enabled: boolean;
  }

  export interface OrgizeSourceBlockNowebPlanDto {
    raw: string;
    source: "explicit" | "default";
    tokens: string[];
    eval: "disabled" | "expand" | "strip";
    export: "disabled" | "expand" | "strip";
    tangle: "disabled" | "expand" | "strip";
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
    resultOptions: OrgizeSourceBlockResultOptionsDto;
    execution: OrgizeSourceBlockExecutionPlanDto;
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
}
