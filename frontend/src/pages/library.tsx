import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createResource, createSignal } from "solid-js";
import UploadCard from "../components/UploadCard";
import { UploadsDataType } from "./uploads.data";
import UploadIcon from "../icons/UploadIcon";
import { AscendingIcon, DescendingIcon } from "../icons/Sorting";
import { getPurchasedUploads } from "../api/uploads";

export default function Library() {
  const myPurchasedUploads = createResource(getPurchasedUploads);
  const [activeSort, setActiveSort] = createSignal("date");
  const [sortDateDirection, setSortDateDirection] = createSignal(false);
  const [sortSizeDirection, setSortSizeDirection] = createSignal(false);
  const [sortDownloadsDirection, setSortDownloadsDirection] = createSignal(false);
  const [sortRatingDirection, setSortRatingDirection] = createSignal(false);

  return (
    <>
      <Link href="/upload" class="btn btn-outline mb-2 md:hidden">
        <UploadIcon />
        Hochladen
      </Link>
      <div class="grid grid-cols-2 gap-2 md:grid-cols-4 max-w-max">
        <button
          class="btn"
          classList={{ "btn-accent": activeSort() === "date" }}
          onClick={() => {
            if (activeSort() === "date") setSortDateDirection(!sortDateDirection());
            setActiveSort("date");
          }}
        >
          <Show when={sortDateDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Datum
        </button>
        <button
          class="btn"
          classList={{ "btn-accent": activeSort() === "size" }}
          onClick={() => {
            if (activeSort() === "size") setSortSizeDirection(!sortSizeDirection());
            setActiveSort("size");
          }}
        >
          <Show when={sortSizeDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Größe
        </button>
        <button
          class="btn"
          classList={{ "btn-accent": activeSort() === "downloads" }}
          onClick={() => {
            if (activeSort() === "downloads") setSortDownloadsDirection(!sortDownloadsDirection());
            setActiveSort("downloads");
          }}
        >
          <Show when={sortDownloadsDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Downloads
        </button>
        <button
          class="btn"
          classList={{ "btn-accent": activeSort() === "rating" }}
          onClick={() => {
            if (activeSort() === "rating") setSortRatingDirection(!sortRatingDirection());
            setActiveSort("rating");
          }}
        >
          <Show when={sortRatingDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Bewertung
        </button>
      </div>

      <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4 mt-6">
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
            when={(myPurchasedUploads()?.length ?? 0) > 0}
            fallback={
              <div class="card card-compact card-side bg-base-200 shadow-md">
                <div class="card-body">
                  <div class="text-center">
                    {/* HACK this looks appalling, improve font/layout */}
                    <h2 class="text-xl font-bold">Keine Uploads gefunden</h2>
                    <p class="text-lg">Sei die erste Person, die hier etwas hochlädt!</p>
                    <Link href="/upload" class="btn btn-sm btn-primary btn-outline mt-2">
                      {/* <UploadIcon /> */}
                      Jetzt Hochladen
                    </Link>
                  </div>
                </div>
              </div>
            }>
            <For each={myPurchasedUploads()}>{(upload) => <UploadCard {...upload} />}</For>
          </Show>
        </Suspense>
      </div>
    </>
  );
}

