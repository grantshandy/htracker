<template>
	<div class="page-root">
		<div v-cloak class="h-root">
            <div class="h-top-text">
                <div class="md:float-left flex space-x-2">
                    <button v-on:click="logout" class="select-none">Logout</button>
                    <p v-if="username" class="font-semibold">Username: {{ username }}</p>
                </div>
                <div class="md:float-right">
                    <ColorSwitcher />
                </div>
            </div>
            <div class="space-y-4">
                <TaskList />
                <MoodLogger />
                <Quote />
            </div>
		</div>
	</div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'
import Quote from '../../components/Quote.vue'
import TaskList from '../../components/TaskList.vue'
import MoodLogger from '../../components/MoodLogger.vue'

export default {
    name: 'Dashboard',
    components: {
        ColorSwitcher,
        Quote,
        TaskList,
        MoodLogger,
    },
    data() {
        return {
            accessToken: null,
            username: null,
            error: null,
        }
    },
    created() {
        this.accessToken = localStorage.getItem('accessToken');

        if (!this.accessToken) {
            window.location.href = '/login';
        }

        // set username from access token
        this.username = atob(this.accessToken).split(':')[0];
    },
    methods: {
        logout() {
            localStorage.removeItem('accessToken');
            window.location.href = '/login';
        }
    }
}
</script>