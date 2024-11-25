import { ErrorResponse, put } from ".";

export interface Upload {
  id: string;
  name: string;
  description: string;
  price: number;
  uploader: string;
  upload_date: string;
  last_modified_date: string;
  belongs_to: string;
  held_by?: string;
}

export type GetUploadsResponse = ErrorResponse | { success: true; uploads: Upload[] };

export interface UploadRequest {
  id?: string;
  name?: string;
  description?: string;
  price?: number;
  belongs_to?: string;
  held_by?: string;
}

export type UploadResponse = ErrorResponse | { success: true; message: string; upload: Upload };

export type FileUploadResponse = ErrorResponse | { success: true; message: string };

export async function getUploads(courseId: string): Promise<Upload[]> {
  const response = await put<GetUploadsResponse>("/api/v1/get/uploads", {
    course_id: courseId,
  });
  if (!response.success) throw new Error(response.message);
  return response.uploads;
}

export async function getUpload(uploadId: string): Promise<Upload> {
  const response = await put<GetUploadsResponse>("/api/v1/get/upload", {
    course_id: uploadId,
  });
  if (!response.success) throw new Error(response.message);
  return response.uploads;
}

export async function upload(options: UploadRequest): Promise<Upload> {
  const response = await put<UploadResponse>("/api/v1/do/upload", options);
  if (!response.success) throw new Error(response.message);
  return response.upload;
}

export async function uploadFile(uploadId: string, file: Blob): Promise<FileUploadResponse> {
  const form = new FormData();
  form.append("upload_id", uploadId);
  form.append("file", file);

  const response = (await (
    await fetch("/api/v1/do/file", {
      method: "PUT",
      body: form,
    })
  ).json()) as FileUploadResponse;
  if (!response.success) throw new Error(response.message);
  return response;
}

export interface PurchaseRequest {
  upload_id: string;
}

export async function purchaseUpload(options: PurchaseRequest): Promise<FileUploadResponse> {
  const response = await put<FileUploadResponse>("/api/v1/do/purchase", options);
  if (!response.success) throw new Error(response.message);
  return response;
}
