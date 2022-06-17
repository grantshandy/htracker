const { createApp } = Vue

createApp({
    data() {
        return {
            login_state: {
                username: null,
                password: null,
            },
            register_state: {
                email: null,
                username: null,
                password: null,
                password_confirm: null,
            },
            error: null,
            info: null,
            registering: false,
            darkmode: true,
        }
    },

    mounted() {
        // update app secret and username from localstorage
        this.secret = localStorage.getItem('secret');

        // enable dark mode if valid
        if (localStorage.getItem('darkmode') == 'true') {
            this.enableDarkMode();
        } else if (localStorage.getItem('darkmode') == 'false') {
            this.disableDarkMode();
        } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        login() {
            console.log('Logging in...');
            this.error = null;
            this.info = null;

            let username = this.login_state.username;
            let password = this.login_state.password;
            
            fetch(window.location.origin + '/api/login', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    username,
                    password,
                }),
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    if (response.valid) {
                        this.setSecret(username, password);
                        window.location.href = '/';
                    } else {
                        this.error = "Incorrect Username or Password";
                    }
                }
            });
        },

        register() {
            this.error = null;
            this.info = null;

            let username = this.register_state.username;
            let password = this.register_state.password;
            let password_confirm = this.register_state.password_confirm;
            let email = this.register_state.email;

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
                    this.register_state.email = null;
                    this.register_state.username = null;
                    this.register_state.password = null;
                    this.register_state.password_confirm = null;
                    this.info = response.info;
                }
            });
        },

        setSecret(username, password) {
            localStorage.setItem('secret', btoa(`${username}:${password}`));
        },

        enableDarkMode() {
            DarkReader.enable({
                brightness: 100,
                contrast: 90,
                sepia: 10
            });
            localStorage.setItem('darkmode', 'true');
        },

        disableDarkMode() {
            DarkReader.disable();
            localStorage.setItem('darkmode', 'false');
        },
    }
})
.mount('#app')