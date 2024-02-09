import { Link } from "@solidjs/router";
import { getUniversities } from "../api/universities";
import { For, createResource } from "solid-js";

export default function Home() {
  const [unis] = createResource(getUniversities);

  return (
    <section>
      <h1 class="text-2xl font-bold">Willkommen bei der Egiraffe</h1>
      <br />
      <Link class="btn btn-outline" href="/courses">Kurse</Link>
      <select class="btn btn-outline ml-2">
        <option>Alle Unis</option>
        <For each={unis()}>
          {(uni) => (
            <option value={uni.id}>{uni.full_name}</option>
          )}
        </For>
      </select>
      <br />
      <h2 class="text-xl font-bold mt-4 mb-1">So geht's</h2>
      <p>
        Suche <em class="italic">oben in der Suchleiste</em>
        nach Kursen, Hausübungen, und Klausuren
        über eine vielzahl an Jahrgängen und Universitäten.
      </p>
    </section>
  );
}
