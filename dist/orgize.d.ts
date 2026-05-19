/* tslint:disable */
/* eslint-disable */

/**
 * WebAssembly wrapper around [`orgize::Org`].
 */
export class Org {
    free(): void;
    [Symbol.dispose](): void;
    agenda(): string;
    agendaBlockJson(request_json: string): string;
    agendaViewJson(request_json: string): string;
    agentMemory(): string;
    agentPlanning(): string;
    attachmentsJson(source_file?: string | null): string;
    capturePlanJson(request_json: string): string;
    clockIssuesJson(request_json?: string | null): string;
    clockRollupsJson(): string;
    clockTablePlansJson(): string;
    columnViewsJson(): string;
    datetreeJson(): string;
    dynamicBlocksJson(): string;
    html(): string;
    includeExpansionJson(base_dir?: string | null): string;
    latex(): string;
    lintJson(): string;
    markdown(): string;
    metadataJson(): string;
    org(): string;
    orgElementsJson(): string;
    outlineJson(): string;
    constructor(input: string);
    progressStatsJson(): string;
    propertyProfileJson(): string;
    refilePlanJson(request_json: string): string;
    refileTargetsJson(request_json?: string | null): string;
    sdd(): string;
    sddJson(): string;
    sectionIndexJson(source_file?: string | null): string;
    semantic(): string;
    snapshotJson(source_file?: string | null): string;
    sourceBlocksJson(): string;
    sparseTreeExplainJson(source_file?: string | null, match_expression?: string | null, text?: string | null, include_archived?: boolean | null): string;
    sparseTreeJson(source_file?: string | null, match_expression?: string | null, text?: string | null, include_archived?: boolean | null): string;
    syntax(): string;
    taskBlockersJson(): string;
    traverse(): string;
    update(s: string): void;
    viewIndexJson(source_file?: string | null): string;
    static readonly buildTime: string;
    static readonly gitHash: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_org_free: (a: number, b: number) => void;
    readonly org_agenda: (a: number) => [number, number];
    readonly org_agendaBlockJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_agendaViewJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_agentMemory: (a: number) => [number, number];
    readonly org_agentPlanning: (a: number) => [number, number];
    readonly org_attachmentsJson: (a: number, b: number, c: number) => [number, number];
    readonly org_buildTime: () => [number, number];
    readonly org_capturePlanJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_clockIssuesJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_clockRollupsJson: (a: number) => [number, number];
    readonly org_clockTablePlansJson: (a: number) => [number, number];
    readonly org_columnViewsJson: (a: number) => [number, number];
    readonly org_datetreeJson: (a: number) => [number, number];
    readonly org_dynamicBlocksJson: (a: number) => [number, number];
    readonly org_gitHash: () => [number, number];
    readonly org_html: (a: number) => [number, number];
    readonly org_includeExpansionJson: (a: number, b: number, c: number) => [number, number];
    readonly org_latex: (a: number) => [number, number];
    readonly org_lintJson: (a: number) => [number, number];
    readonly org_markdown: (a: number) => [number, number];
    readonly org_metadataJson: (a: number) => [number, number];
    readonly org_org: (a: number) => [number, number];
    readonly org_orgElementsJson: (a: number) => [number, number];
    readonly org_outlineJson: (a: number) => [number, number];
    readonly org_parse: (a: number, b: number) => number;
    readonly org_progressStatsJson: (a: number) => [number, number];
    readonly org_propertyProfileJson: (a: number) => [number, number];
    readonly org_refilePlanJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_refileTargetsJson: (a: number, b: number, c: number) => [number, number, number, number];
    readonly org_sdd: (a: number) => [number, number];
    readonly org_sddJson: (a: number) => [number, number];
    readonly org_sectionIndexJson: (a: number, b: number, c: number) => [number, number];
    readonly org_semantic: (a: number) => [number, number];
    readonly org_snapshotJson: (a: number, b: number, c: number) => [number, number];
    readonly org_sourceBlocksJson: (a: number) => [number, number];
    readonly org_sparseTreeExplainJson: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number, number];
    readonly org_sparseTreeJson: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number, number];
    readonly org_syntax: (a: number) => [number, number];
    readonly org_taskBlockersJson: (a: number) => [number, number];
    readonly org_traverse: (a: number) => [number, number];
    readonly org_update: (a: number, b: number, c: number) => void;
    readonly org_viewIndexJson: (a: number, b: number, c: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
