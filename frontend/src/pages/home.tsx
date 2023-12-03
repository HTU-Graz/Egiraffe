import { Link } from "@solidjs/router";

export default function Home() {
  return (
    <section>
      <h1 class="text-2xl font-bold">Home</h1>
      <p class="mt-4">This is the home page.</p>
      <Link href="/courses">Kurse</Link>
    </section>
  );
}
