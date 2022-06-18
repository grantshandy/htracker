const { createApp } = Vue

createApp({
    data() {
        return {
            email: null,
            username: null,
            password: null,
            password_confirm: null,
            error: null,
            info: null,
            darkMode: false,
            registering: false,
        }
    },

    mounted() {
        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        login() {
            window.location.href = '/login';
        },

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
    }
})
.mount('#app')