import { useSignal } from "@preact/signals";
import Counter from "@/islands/Counter.tsx";
import Wallets from "@/islands/Wallets.tsx";
import { add, instantiate } from "@/lib/rs_lib.generated.js";
import { Handlers, PageProps } from "$fresh/server.ts";
import { Fragment } from "preact";

interface Data {
  initial_number: number;
}

export const handler: Handlers<Data | null> = {
  async GET(_request, context) {
    try {
      await instantiate();
      const initial_number = add(2, 2);
      // console.log("add(2+2)", initial_number);
      return context.render({ initial_number });
    } catch (error: unknown) {
      console.error(`Error doing stuff with image file: ${error as string}`);
      return context.render(null);
    }
  },
};

export default function Index(context: PageProps<Data | null>) {
  const { data } = context;
  if (!data) {
    return (
      <Fragment>
        <div>Something went wrong!</div>
      </Fragment>
    );
  }
  const { initial_number } = data;

  const count = useSignal(initial_number);
  return (
    <div class="px-4 py-8 mx-auto">
      <div class="max-w-screen-md mx-auto flex flex-col items-center justify-center">
        <img
          class="my-6"
          src="/logo.svg"
          width="128"
          height="128"
          alt="the Fresh logo: a sliced lemon dripping with juice"
        />
        <h1 class="text-4xl font-bold">Welcome to Fresh</h1>
        <p class="my-4">
          Try updating this message in the
          <code class="mx-2">./routes/index.tsx</code> file, and refresh.
        </p>
        <Counter count={count} />

        <article class="prose lg:prose-xl">
          <h1>Garlic bread with cheese: What the science tells us</h1>
          <p>
            For years parents have espoused the health benefits of eating garlic
            bread with cheese to their children, with the food earning such an
            iconic status in our culture that kids will often dress up as warm,
            cheesy loaf for Halloween.
          </p>
          <p>
            But a recent study shows that the celebrated appetizer may be linked
            to a series of rabies cases springing up around the country.
          </p>
        </article>
        <Wallets count={count} />
      </div>
    </div>
  );
}
