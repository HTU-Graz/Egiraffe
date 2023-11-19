import { Upload } from '../api/uploads';

export default function UploadCard(props: Upload) {
  return (
    <div class="p-4 w-full max-w-4xl">
      <div class="divider"></div>
      <div class="flex items-start">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="inline-block shrink-0 w-20"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          fill="none"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path stroke="none" d="M0 0h24v24H0z" fill="none" />
          <path d="M14 3v4a1 1 0 0 0 1 1h4" />
          <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z" />
          <path d="M9 17h6" />
          <path d="M9 13h6" />
        </svg>
        <div class="inline-block ml-4">
          <h2 class="text-lg font-bold text-blue-600">{props.name}</h2>
          <p>{props.uploader}</p>
          <p class="mt-4">{props.description}</p>
        </div>
      </div>
      <button class="btn btn-primary mt-4 float-right">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="28"
          height="28"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          fill="none"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path stroke="none" d="M0 0h24v24H0z" fill="none" />
          <path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2" />
          <path d="M7 11l5 5l5 -5" />
          <path d="M12 4l0 12" />
        </svg>
        Herunterladen
      </button>
    </div>
  );
}
