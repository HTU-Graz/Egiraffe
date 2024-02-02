import { createResource } from "solid-js";
import { getFiles } from "../api/files";
import { Upload } from "../api/uploads";
import DocumentPreview from "../assets/document-preview.png";
import Rating from "./Rating";

export function bytesToSize(bytes: number): string {
  const units = ["byte", "kilobyte", "megabyte", "gigabyte", "terabyte"];
  const unitIndex = Math.max(
    0,
    Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1),
  );

  return Intl.NumberFormat(navigator.language, {
    style: "unit",
    notation: "compact",
    unit: units[unitIndex],
  }).format(bytes / 1024 ** unitIndex);
}

export default function UploadCard(props: Upload) {
  const [files] = createResource(() => props.id, getFiles);
  const fileName = () => files()?.[0].name;
  const size = () => bytesToSize(files()?.[0].size ?? 0);

  return (
    <div class="card card-compact card-side bg-base-200 shadow-md h-40">
      <figure>
        <img src={DocumentPreview} alt="Document Preview" title={fileName()} class="h-full w-28" />
      </figure>
      <div class="card-body">
        <h2 class="card-title">{props.name}</h2>
        <p>{props.description}</p>
        <div class="flex flex-wrap items-center">
          <svg
            class="inline-block mr-2"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12z" />
            <path d="M16 3v4" />
            <path d="M8 3v4" />
            <path d="M4 11h16" />
            <path d="M11 15h1" />
            <path d="M12 15v3" />
          </svg>
          {new Date(props.upload_date).toLocaleDateString(navigator.language)}
          <span class="mx-2">•</span>
          <svg
            class="inline-block mr-2"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />
            <path d="M14.8 9a2 2 0 0 0 -1.8 -1h-2a2 2 0 1 0 0 4h2a2 2 0 1 1 0 4h-2a2 2 0 0 1 -1.8 -1" />
            <path d="M12 7v10" />
          </svg>
          {props.price} EC<span class="mx-2">•</span>
          <span>{size()}</span>
          <span class="mx-2">•</span>
          <Rating value={0} disabled />
        </div>
      </div>
    </div>
  );
}
