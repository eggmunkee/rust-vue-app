Vue.component('user-form', {
    data: function() {
        return {
            user: {
                name: '',
                age: ''
            }
        };
    },
    template: `
            <div class="user-form">
                <h1>Add User</h1>
                <div style="padding: 10px;">
                    <strong>Name</strong>
                    <input type="text" v-model="user.name"/>
                    <strong>Age</strong>
                    <input type="text" v-model="user.age"/>
                </div>
                <a href="#" @click.prevent="addUser">Add User</a>
            </div>
        `,
    methods: {
        addUser: function() {
            var self = this;
            $.post('/users/', { dataType: 'json', data: {
                name: self.user.name, age: self.user.age
            }}, function(user) {
                console.log('saved user: ', JSON.stringify(user));
                console.log(user.name);
                //self.users = users;
                self.$emit('addUser', user);
            }).fail(function(err) {
                console.log('failure to add: ' + err);
            });
        }
    }
});