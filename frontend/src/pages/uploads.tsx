import { useRouteData } from '@solidjs/router';
import { For, Show, Suspense, createSignal } from 'solid-js';
import UploadCard from '../components/UploadCard';
import { UploadsDataType } from './uploads.data';

export default function Course() {
  const uploads = useRouteData<UploadsDataType>();
  const [activeSort, setActiveSort] = createSignal('date');
  const [sortDateDirection, setSortDateDirection] = createSignal(false);
  const [sortSizeDirection, setSortSizeDirection] = createSignal(false);
  const [sortDownloadsDirection, setSortDownloadsDirection] =
    createSignal(false);
  const [sortRatingDirection, setSortRatingDirection] = createSignal(false);

  return (
    <>
      <div class="flex gap-2 flex-wrap">
        <button
          class="btn"
          classList={{ 'btn-accent': activeSort() === 'date' }}
          onClick={() => {
            setSortDateDirection(!sortDateDirection());
            setActiveSort('date');
          }}
        >
          <Show when={sortDateDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Datum
        </button>
        <button
          class="btn"
          classList={{ 'btn-accent': activeSort() === 'size' }}
          onClick={() => {
            setSortSizeDirection(!sortSizeDirection());
            setActiveSort('size');
          }}
        >
          <Show when={sortSizeDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Größe
        </button>
        <button
          class="btn"
          classList={{ 'btn-accent': activeSort() === 'downloads' }}
          onClick={() => {
            setSortDownloadsDirection(!sortDownloadsDirection());
            setActiveSort('downloads');
          }}
        >
          <Show when={sortDownloadsDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Downloads
        </button>
        <button
          class="btn"
          classList={{ 'btn-accent': activeSort() === 'rating' }}
          onClick={() => {
            setSortRatingDirection(!sortRatingDirection());
            setActiveSort('rating');
          }}
        >
          <Show when={sortRatingDirection()} fallback={DescendingIcon()}>
            {AscendingIcon()}
          </Show>
          Bewertung
        </button>
      </div>

      <Suspense fallback={<span>...</span>}>
        <div class="flex flex-col items-center">
          <For each={uploads()}>{upload => <UploadCard {...upload} />}</For>
        </div>
      </Suspense>
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
