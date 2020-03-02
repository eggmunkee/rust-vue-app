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
                <sub>{{requests}} requests received in shared state.</sub>
                <users-list :user_count="user_count" />
                <user-form />
                <music-player />
            </div>
        `,
        methods: {
            
        }
    });
    window.vm = vm;
}