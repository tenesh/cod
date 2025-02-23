export interface PaginationMeta {
    total: number;
    per_page: number;
    current_page: number;
    last_page: number;
}

export interface PaginatedResponse<T> {
    data: T[];
    pagination: PaginationMeta;
}
