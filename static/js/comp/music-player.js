Vue.component('music-player', {
    data: function() {
        return {
            
        };
    },
    template: `
            <div class="user-form">
                <h1>Example of using npm's vue-plyr and plyr packages for UI functionality.</h1>
                <vue-plyr>
                    <audio>
                        <source src="/static/audio/06 Bubble.mp3" type="audio/mp3"/>
                    </audio>
                </vue-plyr>
            </div>
        `,
    methods: {

    }
});