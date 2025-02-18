import { Link, useRouteData } from "@solidjs/router";
import { For, Show, Suspense, createResource, createSignal } from "solid-js";
import UploadCard from "../components/UploadCard";
import { UploadsDataType } from "./uploads.data";
import UploadIcon from "../icons/UploadIcon";
import { AscendingIcon, DescendingIcon } from "../icons/Sorting";
import { getPurchasedUploads, PurchaseInfoItem } from "../api/uploads";

export default function Library() {
  const [myPurchasedUploads] = createResource(getPurchasedUploads);
  const [activeSort, setActiveSort] = createSignal("date");
  const [sortDateDirection, setSortDateDirection] = createSignal(false);
  const [sortSizeDirection, setSortSizeDirection] = createSignal(false);
  const [sortDownloadsDirection, setSortDownloadsDirection] = createSignal(false);
  const [sortRatingDirection, setSortRatingDirection] = createSignal(false);

  return (
    <>
      <h1 class="text-3xl font-bold">Meine Bibliothek</h1>
      <p class="text-base my-2">
        Hier findest du alle deine Inhalte, die du schon einmal heruntergeladen oder gekauft hast auf einen Blick.
      </p>

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
          <For each={myPurchasedUploads()} fallback={<div>Keine Inhalte gefunden.</div>}>
            {(pi) => <PurchaseInfoCard {...pi} />}
          </For>
        </Suspense>
      </div>
    </>
  );
}

function PurchaseInfoCard(pi: PurchaseInfoItem) {
  return (
    <div class="card card-compact card-side bg-base-200 shadow-md">
      <div class="card-body">
        <h2 class="card-title">{pi.upload.name}</h2>
        <p class="text-sm text-base-content mt-1">{pi.upload.description}</p>
        <div class="flex items-center mt-2">
          <span class="badge badge-outline mr-2">Course: {pi.upload.belongs_to}</span>
          <span class="badge badge-outline">{pi.upload.upload_date}</span>
        </div>
      </div>
    </div>
  );
}
