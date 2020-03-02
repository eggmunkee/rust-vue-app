Vue.component('users-list', {
    props: {
        'user_count': Number
    },
    data: function() {
        return {
            users: []
        };
    },
    template: `
            <div class="users-list">
                <h1>Users in app ({{user_count}})</h1>
                <a href="#" @click.prevent="updateUsers">Update Users</a>
                <div style="padding: 10px;">
                    <ul>
                        <li v-for="user in users" :key="user.rowid">
                            ({{user.rowid}}.) {{user.name}}, {{user.age}}y.o.
                        </li>
                    </ul>
                </div>
            </div>
        `,
    methods: {
        updateUsers: function() {
            var self = this;
            $.get('/users/', { type: 'json' }, function(users) {
                console.log(JSON.stringify(users));
                for (var i=0; i<users.length; i++) {
                    console.log(users[i].name);
                }
                self.users = users;
            });
        }
    }
});