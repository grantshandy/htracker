<template>
	<div class="page-root">
		<div v-cloak class="h-root">
			<div class="h-top-text">
				<a class="md:float-left select-none" href="/">Home</a>
				<div class="md:float-right">
					<ColorSwitcher />
				</div>
			</div>
			<div class="space-y-5">
				<div class="h-box">
					<h1 class="h-title">Login</h1>
					<div class="space-y-3 w-full">
						<div>
							<p>Username:</p>
							<input type="text" class="w-full h-text-input" v-model="username">
						</div>
						<div>
							<p>Password:</p>
							<input type="password" class="w-full h-text-input" v-on:keypress="login" v-model="password">
						</div>
					</div>
					<div class="flow-root text-base-01 dark:text-base-1">
						<button v-on:click="login" class="float-left h-button">Login</button>
						<a class="float-right align-bottom select-none" href="/register">Register Instead</a>
					</div>
				</div>
				<ErrorBox :error="error" v-if="error" v-on:close-box="error = null"/>
			</div>
		</div>
	</div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'
import ErrorBox from '../../components/ErrorBox.vue'

const shajs = require('sha.js');

export default {
	name: 'Login',
	components: {
		ColorSwitcher,
		ErrorBox,
	},
	data() {
		return {
			username: null,
			password: null,
			error: null,
		}
	},
	created() {
		if (localStorage.getItem('sessionToken')) {
			window.location.href = '/dashboard';
		}
	},
	methods: {
		async login(event) {
			// only really run this function when keypressed if we press enter
			if (event && event.key && event.key != 'Enter') {
				return;
			}

			this.error = null;

			fetch(window.location.origin + '/api/login', {
				method: 'POST',
				body: JSON.stringify({
					username: this.username,
					password: shajs('sha256').update(this.password).digest('hex'),
				}),
				headers: {
					'Accept': 'application/json',
				},
			})
			.then(response => response.json())
			.then(response => {
				if (response.error) {
					this.error = response.error;
				} else {
					localStorage.setItem('sessionToken', response.sessionToken);
					window.location.href = '/dashboard';
				}
			})
			.catch(error => {
                this.error = error.message;
            });
		}
	}
}
</script>
