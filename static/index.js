console.log('test');

function index(el, pageData) {
    let vm = new Vue({
        el: el,
        data: pageData,
        template: `
            <div class="index-app">
                <h2>Index content</h2>
                <p>Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis.</p>
                <p>Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis.</p>
                <h1 style="background-color: yellow;">{{requests}}</h1>
                <h1>User count: {{user_count}}</h1>
                <ul>
                    <div v-if="users.length>0">
                        <div v-for="(user,i) in users">
                            <div>
                                ({{i}})
                                <strong>Name:</strong> {{ user.name }} 
                            </div>
                            <div>
                                <strong>Age:</strong> {{ user.age }} 
                            </div>
                        </div>
                    </div>
                    <div v-else style="color:888;">No users</div>
                </ul>
                <textarea>
                    {{ users }}
                </textarea>
                <hr/>
                <p>Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis. Test ipsul lorem semper fidelis.</p>
                <h1>Users in app</h1>
                <h3>
                    <ul>
                        <li v-for="user in users" :key="user.rowid">
                            ({{user.rowid}}.) {{user.name}}, {{user.age}}y.o.
                        </li>
                    </ul>
                </h3>
            </div>
        `
    });
}