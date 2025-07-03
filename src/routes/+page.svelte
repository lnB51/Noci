<script lang="ts">
	// @ts-nocheck
	import { onMount, onDestroy } from 'svelte';
	import { writable } from 'svelte/store';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';
	import close from '$lib/assets/close.svg';
	import nociIcon from '$lib/assets/noci_icon.svg';
	import play from '$lib/assets/play.svg';
	import prev from '$lib/assets/prev.svg';
	import next from '$lib/assets/next.svg';
	import pause from '$lib/assets/pause.svg';

	let music_info = null;
	let lastTrackInfo = null;

	let bars = [6, 12, 18, 6];
	let animating = false;
	let animationFrameId: number | null = null;
	let barColor = '#4ade80';

	let isDragging = false;
	let progressBarEl: HTMLDivElement;
	const windowWidth = writable(0);

	// Reactive derived values
	$: isPlaying = music_info?.player_state === 'playing';
	$: displayTrack = music_info?.track_name ? music_info : lastTrackInfo;
	$: progressPercent =
		displayTrack?.position && displayTrack?.track_duration
			? Math.min(100, Math.max(0, (displayTrack.position / displayTrack.track_duration) * 100))
			: 0;

	// Playback controls config
	const controls = [
		{ icon: prev, label: 'Previous track', handler: previousTrack },
		{
			icon: () => (isPlaying ? pause : play),
			label: () => (isPlaying ? 'Pause' : 'Play'),
			handler: playPause
		},
		{ icon: next, label: 'Next track', handler: nextTrack }
	];

	// Seek/drag behavior
	function handleSeek(e: MouseEvent) {
		if (!displayTrack?.track_duration) return;
		const { left, width } = progressBarEl.getBoundingClientRect();
		const clampedX = Math.min(Math.max(0, e.clientX - left), width);
		const newPosition = (clampedX / width) * displayTrack.track_duration;

		displayTrack.position = newPosition;
		invoke('set_track_position', { position: newPosition }).catch(console.error);
	}

	function startDragging(e: MouseEvent) {
		isDragging = true;
		handleSeek(e);

		const move = (e: MouseEvent) => isDragging && handleSeek(e);
		const up = () => {
			isDragging = false;
			window.removeEventListener('mousemove', move);
			window.removeEventListener('mouseup', up);
		};

		window.addEventListener('mousemove', move);
		window.addEventListener('mouseup', up);
	}

	// Audio bar animation
	function animateBars(time = 0) {
		bars = animating
			? bars.map((_, i) => 15 + 9 * Math.sin(time / 300 + i * 1.2))
			: bars.map(() => 6);

		animationFrameId = requestAnimationFrame(animateBars);
	}

	// Time formatting
	function formatTime(seconds?: number) {
		if (!seconds || seconds < 0) return '0:00';
		const m = Math.floor(seconds / 60);
		const s = Math.floor(seconds % 60);
		return `${m}:${s.toString().padStart(2, '0')}`;
	}

	// Playback control handlers
	function playPause() {
		invoke('toggle_playback').catch(console.error);
	}
	function nextTrack() {
		invoke('next_track').catch(console.error);
	}
	function previousTrack() {
		invoke('previous_track').catch(console.error);
	}

	// Lifecycle hooks
	onMount(async () => {
		// Handle resize
		windowWidth.set(window.innerWidth);
		const onResize = () => windowWidth.set(window.innerWidth);
		window.addEventListener('resize', onResize);

		// Init fast average color
		const facModule = await import('fast-average-color');
		const FastAverageColor = facModule.default || facModule.FastAverageColor || facModule;
		const fac = new FastAverageColor();

		// Listen for Spotify events
		listen('spotify-status-update', async (event) => {
			const payload = event.payload;
			console.log('Spotify status update:', payload);

			music_info = payload;
			if (payload?.track_name) lastTrackInfo = payload;
			animating = payload?.player_state === 'playing';

			if (payload?.album_cover) {
				try {
					const { hex } = await fac.getColorAsync(payload.album_cover);
					barColor = hex;
				} catch {
					barColor = '#4ade80';
				}
			}
		});

		animateBars();

		// Cleanup
		onDestroy(() => {
			window.removeEventListener('resize', onResize);
			if (animationFrameId !== null) {
				cancelAnimationFrame(animationFrameId);
			}
		});
	});
</script>

<svelte:head>
	<link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded" rel="stylesheet" />
</svelte:head>

{#if displayTrack}
	{#if $windowWidth > 450}
		<div class="flex h-[10vh] items-center justify-between bg-black px-4 py-5 text-white">
			<div class="flex items-center gap-2">
				<img src={nociIcon} alt="Noci logo" class="h-6 w-6" />
				<span class="font-semibold">Noci</span>
			</div>
			<button
				class="exit-button flex cursor-pointer items-center justify-center rounded p-1 transition hover:bg-red-600"
				aria-label="Exit app"
				on:click={() => {
					invoke('exit_app').catch(console.error);
				}}
			>
				<img src={close} alt="Close" class="h-6 w-6" />
			</button>
		</div>
		<div
			class="mx-auto flex h-[85vh] max-w-3xl items-center justify-center gap-4 rounded-b-lg bg-black px-4 py-4 text-white"
			style="--bar-color: {barColor}"
		>
			<img
				src={displayTrack.album_cover}
				alt="Album Cover"
				class="h-2/3 w-auto rounded-lg object-cover"
			/>

			<div class="flex flex-1 flex-col items-center justify-center">
				<div class="w-full">
					<div class="text-center text-lg font-semibold">{displayTrack.track_name}</div>
					<div class="text-center text-sm text-gray-400">
						{displayTrack.artist_name || 'Unknown Artist'}
					</div>

					<!-- svelte-ignore a11y_interactive_supports_focus -->
					<div
						bind:this={progressBarEl}
						class="relative mt-2 h-1 cursor-pointer overflow-hidden rounded bg-gray-800"
						role="slider"
						aria-valuemin="0"
						aria-valuemax={displayTrack.track_duration || 100}
						aria-valuenow={displayTrack.position || 0}
						aria-label="Playback progress"
						on:mousedown={startDragging}
					>
						<div
							class="h-full transition-all duration-200"
							style="width: {progressPercent}%; background-color: var(--bar-color);"
						></div>
					</div>

					<div class="mt-1 flex justify-between text-xs text-gray-400 select-none">
						<div>{formatTime(displayTrack.position)}</div>
						<div>{formatTime(displayTrack.track_duration)}</div>
					</div>
				</div>

				<div class="mt-3 flex gap-4">
					{#each controls as control}
						<button
							class="flex cursor-pointer items-center justify-center rounded-full p-2 transition hover:bg-[var(--bar-color)]"
							on:click={control.handler}
							aria-label={typeof control.label === 'function' ? control.label() : control.label}
						>
							<img
								src={typeof control.icon === 'function' ? control.icon() : control.icon}
								alt=""
								class="h-6 w-6"
							/>
						</button>
					{/each}
				</div>
			</div>

			<div class="flex h-15 w-15 items-center justify-center gap-1">
				{#each bars as height}
					<div
						class="bar max-h-15 w-1.5 rounded-full"
						style="height: {height + 20}px; margin-top: {-height /
							2}px; background-color: {barColor};"
					></div>
				{/each}
			</div>
		</div>
	{:else}
		<div class="flex h-[32px] w-full items-center justify-between bg-black px-2">
			<img src={displayTrack.album_cover} alt="" class="h-6 w-6 rounded-[5px] object-cover" />
			<div class="flex h-3 w-5 items-center gap-1">
				{#each bars as height}
					<div
						class="bar max-h-3 w-1 rounded-full"
						style="height: {height}px; margin-top: {-height / 2}px; background-color: {barColor};"
					></div>
				{/each}
			</div>
		</div>
	{/if}
{/if}

<style>
	.bar {
		transition: height 0.15s ease;
	}

	.material-symbols-rounded {
		font-family: 'Material Symbols Rounded';
		font-weight: normal;
		font-style: normal;
		font-size: 30px;
		display: inline-block;
		letter-spacing: normal;
		white-space: nowrap;
		direction: ltr;
		-webkit-font-smoothing: antialiased;
	}
</style>
