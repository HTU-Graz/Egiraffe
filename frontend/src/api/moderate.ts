import { put } from ".";
import { GetUploadsResponse } from "./uploads";

export interface ModifyFileRequest {
    id: string;
    name?: string;
    mime_type?: string;
    /** The latest one should match the file's last modified date */
    revision_at?: string;
    /** The ID of the upload this file belongs to */
    upload_id?: string;
    approval_mod?: boolean;
}


export async function mod_modifyUpload(options: ModifyFileRequest): Promise<Upload> {
    const response = await put<ModifyFileRequest>("/api/v1/mod/content/modify-file", options);
    if (!response.success) throw new Error(response.message);
    return response.upload;
}


