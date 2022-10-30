<script lang="ts">
	import init, { get_n_words } from 'rnd-word/pkg/rnd_word';

	type Word = {
		text: string;
		selected: boolean;
	};

	let words: Word[] = [];

	function handleClick() {
		const selectedWords = words.filter((w) => w.selected);

		let newWords: Word[] = get_n_words(5 - selectedWords.length)
			.split(':')
			.map((word) => ({ text: word, selected: false }));

		words = [...selectedWords, ...newWords];
	}
</script>

<main>
	<div class="mb-4">
		<h1>5 Random Words</h1>
		<div>Get five random words to spark some interesting project ideas..</div>
	</div>

	<!-- Show loading until wasm module is loaded.. -->
	{#await init()}
		<div>Loading....</div>
	{:then _}
		<button class="mb-4" on:click={handleClick}>Get random words</button>

		{#if words.length}
			<div class="d-flex border p-6">
				<h4>Generated words:</h4>

				{#each words as word}
					<button
						class="light"
						class:selected={word.selected}
						on:click={() => (word.selected = !word.selected)}
						>{word.text}</button
					>
				{/each}
			</div>
		{/if}
	{/await}
</main>

<style>
	.mb-4 {
		margin-bottom: 1rem;
	}

	.p-6 {
		padding: 1.5rem;
	}

	.light {
		color: black;
		padding: 5px 10px;
		border-radius: 20px;
		background-color: rgba(255, 255, 255, 0.5);
	}

	.d-flex {
		display: flex;
		gap: 1rem;
		justify-content: center;
		align-items: center;
	}

	.selected {
		background: #ccc;
		color: black;
	}

	.border {
		border: 2px solid gray;
		border-radius: 25px;
	}
</style>
