<template>
	<div v-cloak class="page-root">
		<div class="h-root">
			<div class="h-top-text">
				<div class="md:float-right">
					<ColorSwitcher />
				</div>
			</div>
			<div class="h-box text-center">
				<div>
					<h1 class="h-title">htracker</h1>
					<p>A minimalist habit tracker and to-do list. (WIP)</p>
				</div>
				<div class="flow-root text-base-01 dark:text-base-1">
					<a class="float-left" href="/login">Login</a>
					<a class="float-right" href="/register">Register</a>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'

export default {
	name: 'Main',
	components: {
		ColorSwitcher,
	},
	created() {
		if (localStorage.getItem('sessionToken')) {
			window.location.href = '/dashboard'
		}
	},
}
</script>
