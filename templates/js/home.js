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
