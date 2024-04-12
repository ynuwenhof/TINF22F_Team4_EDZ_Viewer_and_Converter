export interface ArchiveInformation {
    id: string,
    name: string,
    size: number,
    created: number,
    status: string,
    packages?: number,
    expires?: number
}
