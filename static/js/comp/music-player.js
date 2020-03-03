Vue.component('music-player', {
    data: function() {
        return {
            src: ''
        };
    },
    template: `
            <div class="user-form">
                <h1>Example of using npm's vue-plyr and plyr packages for UI functionality.</h1>
                <vue-plyr>
                    <audio>
                        <source :src="src" type="audio/mp3"/>
                    </audio>
                </vue-plyr>
            </div>
        `,
    methods: {

    }
});