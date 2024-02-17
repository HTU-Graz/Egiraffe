import { put } from ".";
import { GetUploadsResponse, Upload } from "./uploads";
import { File } from "./files";

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

export async function mod_getAllUploads(): Promise<Upload[]> {
    const response = await put<Upload[]>("/api/v1/mod/content/get-all-uploads");
    if (!response.success) throw new Error(response);
    return response.uploads;
}

export async function mod_modifyUpload(options: ModifyFileRequest): Promise<Upload> {
    const response = await put<ModifyFileRequest>("/api/v1/mod/content/modify-file", options);
    if (!response.success) throw new Error(response.message);
    return response.upload;
}

export async function mod_getAllFiles(): Promise<File[][]> {
    const response = await put<Upload[]>("/api/v1/mod/content/get-all-files");
    if (!response.success) throw new Error(response);
    return response.files;
}
