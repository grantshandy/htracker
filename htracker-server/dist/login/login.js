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
            darkMode: false,
            registering: false,
        }
    },

    mounted() {
        // update app accessToken and username from localstorage
        this.accessToken = localStorage.getItem('accessToken');

        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        login() {
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
                        console.log('sending to dashboard');
                        this.setAccessToken(username, password);
                        window.location.href = '/dashboard';
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

        enableDarkMode() {
            this.darkMode = true;
            localStorage.setItem('darkMode', 'true');
            DarkReader.enable({
                brightness: 100,
                contrast: 90,
                sepia: 10
            });
        },

        disableDarkMode() {   
            this.darkMode = false;
            localStorage.setItem('darkMode', 'false');         
            DarkReader.disable();
        },

        setAccessToken(username, password) {
            localStorage.setItem('accessToken', btoa(`${username}:${password}`));
        },
    }
})
.mount('#app')