import { ErrorResponse, put } from ".";

export interface Upload {
  id: string;
  name: string;
  description: string;
  price: number;
  uploader: string;
  upload_date: Date;
  last_modified_date: Date;
  belongs_to: string;
  held_by?: string;
}

export type GetUploadResponse = ErrorResponse | { success: true; upload: Upload };
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

type PurchaseInfoResponse = ErrorResponse | { success: true; purchase_info_items: PurchaseInfoItem[] };

export async function getUploads(courseId: string): Promise<Upload[]> {
  const response = await put<GetUploadsResponse>("/api/v1/get/uploads", {
    course_id: courseId,
  });
  if (!response.success) throw new Error(response.message);
  for (const ul of response.uploads) {
    ul.upload_date = new Date(ul.upload_date);
    ul.last_modified_date = new Date(ul.last_modified_date);
  }
  return response.uploads;
}

export async function getUpload(uploadId: string): Promise<Upload> {
  const response = await put<GetUploadResponse>("/api/v1/get/upload", {
    course_id: uploadId,
  });
  if (!response.success) throw new Error(response.message);
  response.upload.upload_date = new Date(response.upload.upload_date);
  response.upload.last_modified_date = new Date(response.upload.last_modified_date);
  return response.upload;
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

export interface Purchase {
  user_id: string;
  upload_id: string;
  ecs_spent: number;
  purchase_date: Date;
  rating: number | null;
}

export interface PurchaseInfoItem {
  purchase: Purchase;
  upload: Upload;
  most_recent_available_file: File;
}

export async function getPurchasedUploads(): Promise<PurchaseInfoItem[]> {
  const response = await put<PurchaseInfoResponse>("/api/v1/get/purchased-uploads");
  if (!response.success) throw new Error(response.message);
  return response.purchase_info_items;
}
