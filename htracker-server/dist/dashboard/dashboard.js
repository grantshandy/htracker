const { createApp } = Vue

createApp({
    data() {
        return {
            accessToken: null,
            data: null,
            darkMode: false,
            error: null,
            currentTodo: null,
            loading: false,
        }
    },

    created() {
        this.accessToken = localStorage.getItem('accessToken');

        if (!this.accessToken) {
            window.location.href = '/login';
        }

        if (localStorage.getItem('darkMode') == 'true') {
            this.enableDarkMode();
        } else {
            this.disableDarkMode();
        }
    },

    async mounted() {
        await this.updateData();
    },

    methods: {
        logout() {
            localStorage.removeItem('accessToken');
            window.location.href = '/';
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

        async updateData() {
            await fetch(window.location.origin + '/api/get_data', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                } else {
                    this.data = response;
                }
            });
        },

        async addTodo(event) {
            if ((event && event.key && event.key != 'Enter') || this.currentTodo == '' || !this.currentTodo) {
                return;
            }

            this.loading = true;

            await fetch(window.location.origin + '/api/add_todo', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'X-AuthToken': this.accessToken,
                },
                body: JSON.stringify({
                    name: this.currentTodo,
                }),
            })
            .then(response => response.json())
            .then(response => {
                if (response.error) {
                    this.error = response.error;
                }
            });

            this.currentTodo = '';
            await this.updateData();
        
            this.loading = false;
        }
    }
})
.mount('#app');