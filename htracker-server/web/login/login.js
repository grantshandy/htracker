const { createApp } = Vue

createApp({
    data() {
        return {
            username: null,
            password: null,
            error: null,
            darkMode: false,
            registering: false,
        }
    },

    mounted() {
        this.accessToken = localStorage.getItem('accessToken');

        if (this.accessToken) {
            window.location.href = '/dashboard';
        }

        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    methods: {
        login(event) {
            // only really run this function when keypressed if we press enter
            if (event && event.key && event.key != 'Enter') {
                return;
            }

            this.error = null;

            let username = this.username;
            let password = this.password;

            console.log(this.genAccessToken(username, password));
            
            fetch(window.location.origin + '/api/auth', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.genAccessToken(username, password),
                },
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    if (response.valid) {
                        this.setAccessToken(username, password);
                        window.location.href = '/dashboard';
                    } else {
                        this.error = "incorrect username or password";
                    }
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
            localStorage.setItem('accessToken', this.genAccessToken(username, password));
        },

        genAccessToken(username, password) {
            return btoa(`${username}:${password}`);
        }
    }
})
.mount('#app')