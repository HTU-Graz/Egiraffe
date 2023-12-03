
export interface File {
    id: string;
    name: string;
    mime_type: string;
    size: number;
    revision_at: string;
    /** The ID of the upload this file belongs to */
    upload_id: string;
}
