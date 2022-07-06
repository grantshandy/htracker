<template>
	<div class="page-root">
		<div v-cloak class="h-root">
            <div class="h-top-text">
                <div class="md:float-left flex space-x-2">
                    <button v-on:click="logout" class="select-none">Logout</button>
                </div>
                <div class="md:float-right">
                    <ColorSwitcher />
                </div>
            </div>
            <div class="space-y-4">
                <TaskList />
                <Quote />
            </div>
		</div>
	</div>
</template>

<script>
import ColorSwitcher from '../../components/ColorSwitcher.vue'
import Quote from '../../components/Quote.vue'
import TaskList from '../../components/TaskList.vue'

export default {
    name: 'Dashboard',
    components: {
        ColorSwitcher,
        Quote,
        TaskList,
    },
    data() {
        return {
            sessionToken: null,
            username: null,
        }
    },
    created() {
        this.sessionToken = localStorage.getItem('sessionToken');

        if (!this.sessionToken) {
            window.location.href = '/login';
        }
    },
    methods: {
        async logout() {
            await fetch(window.location.origin + '/api/logout', {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'X-SessionToken': localStorage.getItem('sessionToken'),
                }
            })

            localStorage.removeItem('sessionToken');
            window.location.href = '/login';
        }
    }
}
</script>