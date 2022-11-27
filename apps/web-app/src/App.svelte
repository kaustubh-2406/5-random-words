<script lang="ts">
	import init, { get_n_words } from 'rnd-word/pkg/rnd_word';
	import Heart from './component/Heart.svelte';

	type Word = {
		text: string;
		selected: boolean;
	};

	type PanelOptions = 'description' | 'details';

	let active: PanelOptions = 'description';
	let words: Word[] = [];

	function handleClick() {
		const selectedWords = words.filter((w) => w.selected);

		const n = 5 - selectedWords.length;
		if (n > 0) {
			let newWords: Word[] = get_n_words(n)
				.split(':')
				.map((word) => ({ text: word, selected: false }));

			words = [...selectedWords, ...newWords];
		}
	}
</script>

<section
	class="text-gray-400 bg-gray-900 body-font container px-5 py-24 m-auto"
>
	<div>
		<div class="flex justify-between items-center">
			<div>
				<h2 class="text-sm title-font text-gray-500 tracking-widest">
					<a
						target="_blank"
						rel="noreferrer"
						class="underline"
						href="https://github.com/kaustubh-2406"
					>
						Kaustubh
					</a>
				</h2>
				<h1 class="text-white text-3xl title-font font-medium mb-4">
					5 Random Words
				</h1>
			</div>

			<div class="sm:hidden">
				{#await init()}
					<div class="text-white">Loading WASM binaries....</div>
				{:then _}
					<button
						class="text-white bg-purple-500 border-0 py-2 px-6 focus:outline-none hover:bg-purple-600 rounded"
						on:click={handleClick}
					>
						Get random words
					</button>
				{/await}
			</div>
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
			<div class="hidden sm:inline-block">
				<!-- panel -->
				<div class="flex mb-4">
					<button
						class="flex-grow border-gray-800 border-b-2 py-2 text-lg px-1"
						class:pane-selected={active === 'description'}
						on:click={() => (active = 'description')}
					>
						Description
					</button>
					<button
						class="flex-grow border-b-2 border-gray-800 py-2 text-lg px-1"
						class:pane-selected={active === 'details'}
						on:click={() => (active = 'details')}
					>
						Details
					</button>
				</div>

				<!-- Content -->
				<div class="leading-relaxed h-60">
					{#if active === 'description'}
						<p>
							You can click on words to lock them, and thengenerate another set
							of random words.
						</p>

						<button
							class="mt-8 text-white bg-purple-500 border-0 py-2 px-6 focus:outline-none hover:bg-purple-600 rounded"
							on:click={handleClick}
						>
							Get random words
						</button>
					{/if}

					{#if active === 'details'}
						<p>
							Hey! You found us. This helper would try and suggest you 5 random
							words that you might use for starting your next project.
						</p>
						<p>
							You could also take a look at
							<a
								target="_blank"
								class="underline"
								rel="noopener noreferrer"
								href="http://www.github.com/kaustubh-2406/5-random-words"
							>
								Github repo
							</a>
							for cli and upcoming changes.
						</p>
					{/if}
				</div>
			</div>
			<div
				class="h-60 rounded-lg bg-gray-800 bg-opacity-40 flex justify-center items-center"
			>
				<div class="justify-center items-center">
					{#await init()}
						<div class="text-white">Loading....</div>
					{:then _}
						{#if words.length}
							<div class="flex justify-center items-center p-6 gap-2 flex-wrap">
								{#each words as word}
									<button
										class="color-black p-2 px-6 rounded-lg flex justify-between items-center  transition-all duration-100"
										class:selected={word.selected}
										on:click={() => (word.selected = !word.selected)}
									>
										<span>{word.text}</span>
										<Heart selected={word.selected} />
									</button>
								{/each}
							</div>
						{:else}
							<div class="w-full">No words selected!</div>
						{/if}
					{/await}
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.pane-selected {
		@apply text-purple-400 border-purple-500;
	}
	.selected {
		@apply bg-green-400 text-black;
	}
</style>
