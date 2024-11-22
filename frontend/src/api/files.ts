import { ErrorResponse, put } from ".";
import { Upload } from "./uploads";

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

export interface UploadAndFiles {
  upload: Upload;
  files: File[];
  total_files_count: number;
}

export type FilesResponse =
  | ErrorResponse
  | {
    success: true;
    files: File[];
    upload: Upload;
    total_files_count: number;
  };

export async function getFiles(uploadId: string): Promise<UploadAndFiles> {
  const response = await put<FilesResponse>("/api/v1/get/files-of-upload", { upload_id: uploadId });
  if (!response.success) throw new Error(response.message);
  return {
    upload: response.upload,
    files: response.files,
    total_files_count: response.total_files_count,
  }
}
