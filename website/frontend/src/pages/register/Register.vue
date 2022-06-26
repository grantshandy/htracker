<template>
	<div class="page-root">
		<div v-cloak class="h-root">
			<div class="h-top-text">
				<a href="/">Home</a>
				<div class="float-right">
					<ColorSwitcher />
				</div>
			</div>
			<div class="space-y-5">
				<div class="h-box">
					<h1 class="h-title">Register</h1>
					<div class="space-y-3 w-full">
						<div>
							<p>Email:</p>
							<input type="text" class="w-full h-text-input" v-model="email">
						</div>
						<div>
							<p>Username:</p>
							<input type="text" class="w-full h-text-input" v-model="username">
							<p v-if="!username || (username && username.length < 4)" class="text-sm italic">Must be at least 4 characters.</p>
						</div>
						<div>
							<p>Password:</p>
							<input type="password" class="w-full h-text-input" v-model="password">
							<p v-if="!password || (password && password.length < 6)" class="text-sm italic">Must be at least 6 characters.</p>
						</div>
						<div>
							<p>Confirm Password:</p>
							<input type="password" class="w-full h-text-input" v-model="password_confirm">
							<p v-if="!password_confirm || (password_confirm && password_confirm.length < 6)" class="text-sm italic">Must be at least 6 characters.</p>
							<p v-if="password != password_confirm" class="text-sm italic">Passwords must match.</p>
						</div>
					</div>
					<div class="flow-root text-base-01 dark:text-base-1">
						<button v-on:click="register" class="float-left h-button">Register</button>
						<a class="float-right align-bottom" href="/login">Login Instead</a>
					</div>
				</div>
				<ErrorBox :error="error" v-if="error" v-on:close-box="error = null"/>
				<InfoBox :info="info" v-if="info" v-on:close-box="info = null"/>
			</div>
		</div>
	</div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'
import ErrorBox from '../../components/ErrorBox.vue'
import InfoBox from '../../components/InfoBox.vue'

export default {
	name: 'Register',
	components: {
		ColorSwitcher,
		ErrorBox,
		InfoBox,
	},
	data() {
		return {
			email: null,
			username: null,
			password: null,
			password_confirm: null,
			error: null,
			info: null,
			darkMode: false,
		}
	},
	methods: {
		register() {
			this.error = null;
			this.info = null;

			let username = this.username;
			let password = this.password;
			let password_confirm = this.password_confirm;
			let email = this.email;

			if (!username || username.length < 4) {
				this.error = "Username Must Be at Least 4 Characters.";
				return;
			}

			if (!password || password.length < 6) {
				this.error = "Password Must Be at Least 6 Characters."
				return;
			}

			if ((!password || !password_confirm) || (password_confirm != password)) {
				this.error = "Passwords Must Match";
				return;
			}

			this.info = "Loading...";

			fetch(window.location.origin + '/api/register', {
				method: 'POST',
				headers: {
					'Accept': 'application/json',
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username,
					password,
					email,
				}),
			})
			.then(response => response.json())
			.then(response => {
				if (response.error) {
					this.error = response.error;
					this.info = null;
				} else if (response.info) {
					this.email = null;
					this.username = null;
					this.password = null;
					this.password_confirm = null;
					this.info = response.info;
				}
			});
		},
	}
}
</script>