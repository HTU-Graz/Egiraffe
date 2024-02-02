import { ErrorResponse, put } from ".";

export interface File {
  id: string;
  name: string;
  mime_type: string;
  size: number;
  revision_at: string;
  upload_id: string;
  approval_uploader: boolean;
  approval_mod: boolean;
}

export type FilesResponse =
  | ErrorResponse
  | {
      success: true;
      files: File[];
    };

export async function getFiles(uploadId: string): Promise<File[]> {
  const response = await put<FilesResponse>("/api/v1/get/files", { upload_id: uploadId });
  if (!response.success) throw new Error(response.message);
  return response.files;
}
