import { useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createSignal } from "solid-js";
import UploadCard from "../components/UploadCard";
import { UploadsDataType } from "./uploads.data";

export default function Upload() {
  // const uploads = useRouteData<UploadsDataType>();
  // const [activeSort, setActiveSort] = createSignal("date");
  // const [sortDateDirection, setSortDateDirection] = createSignal(false);
  // const [sortSizeDirection, setSortSizeDirection] = createSignal(false);
  // const [sortDownloadsDirection, setSortDownloadsDirection] = createSignal(false);
  // const [sortRatingDirection, setSortRatingDirection] = createSignal(false);

  return (
    <>
      <div class="flex gap-2 flex-wrap">
        Upload info go brrr
      </div>

      {/* <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4 mt-6">
        <Suspense
          fallback={
            <For each={Array(9)}>
              {() => (
                <div class="card card-compact card-side bg-base-200 shadow-md h-40">
                  <div class="skeleton h-full w-28"></div>
                  <div class="card-body">
                    <div class="skeleton h-6 w-48"></div>
                    <div class="skeleton h-4 w-full"></div>
                    <div class="skeleton h-4 w-full"></div>
                    <div class="skeleton h-4 w-64"></div>
                  </div>
                </div>
              )}
            </For>
          }
        >
          <Show
            when={(uploads()?.length ?? 0) > 0}
            fallback={
              <div class="card card-compact card-side bg-base-200 shadow-md">
                <div class="card-body">
                  <div class="text-center"> */}
      {/* HACK this looks appalling, improve font/layout */}
      {/* <h2 class="text-3xl font-bold">Keine Uploads gefunden</h2>
                  </div>
                </div>
              </div>
            }>
            <For each={uploads()}>{(upload) => <UploadCard {...upload} />}</For>
          </Show>
        </Suspense>
      </div> */}
    </>
  );
}

const AscendingIcon = () => (
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
    <path d="M4 6l7 0" />
    <path d="M4 12l7 0" />
    <path d="M4 18l9 0" />
    <path d="M15 9l3 -3l3 3" />
    <path d="M18 6l0 12" />
  </svg>
);

const DescendingIcon = () => (
  <svg
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
    <path d="M4 6l9 0" />
    <path d="M4 12l7 0" />
    <path d="M4 18l7 0" />
    <path d="M15 15l3 3l3 -3" />
    <path d="M18 6l0 12" />
  </svg>
);
