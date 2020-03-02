console.log('test');

function index(el, pageData) {
    let vm = new Vue({
        el: el,
        data: function() {
            let data = {
                // add in any addition state to page data
            };
            return Object.assign(data, pageData);
        },
        template: `
            <div class="index-app">
                <h1 style="background-color: yellow;">{{requests}}</h1>
                <users-list :user_count="user_count" />
                <user-form />
            </div>
        `,
        methods: {
            
        }
    });
    window.vm = vm;
}