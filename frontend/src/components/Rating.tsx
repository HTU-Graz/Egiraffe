import { For, createUniqueId } from "solid-js";

interface Props {
  /**
   * 0 to 5 (inclusive)
   */
  value: number;
  disabled?: boolean;
  onClick?: (value: number) => void;
}

export default function Rating(props: Props) {
  const name = createUniqueId();

  return (
    <div class="rating rating-sm">
      <input type="radio" name={name} class="rating-hidden hidden" checked={props.value === 0} />

      <For each={[1, 2, 3, 4, 5]}>
        {(i) => (
          <input
            type="radio"
            name={name}
            class="mask mask-star-2 bg-orange-400"
            onClick={() => props.onClick?.(i)}
            disabled={props.disabled}
            checked={props.value === i}
          />
        )}
      </For>
    </div>
  );
}
