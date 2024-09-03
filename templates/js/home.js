$("button[id^='del-item-id-']").on('click', function(e) {
    const data_id = $(e.currentTarget.parentElement).attr("data-id");
    $.ajax({
        url: '/movie/' + data_id,
        method: 'DELETE',
        success: function(response) {
            console.log(response);
            location.reload();
        },
        error: function(xhr, status, error) {
            console.error(status, error);
        }
    });
});

$("button[id^='letterboxd-item-id-']").on('click', function(e) {
    const tmdb_id = $(e.currentTarget.parentElement).attr("data-tmdb-id");
    //document.location.href="https://letterboxd.com/tmdb/"+tmdb_id;
    window.open("https://letterboxd.com/tmdb/"+tmdb_id, "_blank").focus();
});

$("button[id^='overseerr-item-id-']").on('click', function(e) {
    const tmdb_id = $(e.currentTarget.parentElement).attr("data-tmdb-id");
    window.location.href="http://192.168.2.40:5055/movie/"+tmdb_id;
});
