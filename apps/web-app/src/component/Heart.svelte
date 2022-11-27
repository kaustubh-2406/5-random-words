<script type="ts">
	export let selected = false;
</script>

<!-- Heart -->
<span
	class="rounded-full w-10 h-10 bg-transparent p-0 border-0 inline-flex items-center justify-center text-gray-500 ml-4"
>
	<svg
		fill="currentColor"
		stroke-linecap="round"
		stroke-linejoin="round"
		stroke-width="2"
		viewBox="0 0 24 24"
		class="w-4 h-4"
	>
		<path
			class="w-3 h-3 opacity-0 transition-all delay-75 duration-100"
			class:active={selected}
			class:opacity-100={selected}
			d="M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z"
		/>
	</svg>
</span>

<style>
	.active {
		fill: red;
	}
</style>
