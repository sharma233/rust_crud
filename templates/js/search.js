$("button[id^='add-item-id-']").on('click', function(e) {
    const data_id = parseInt($(e.currentTarget.parentElement).attr("data-id"));
    const data_name = $(e.currentTarget.parentElement).attr("data-name");
    const payload = JSON.stringify({
        id: data_id,
        name: data_name
    });
    console.log(payload);
    $.ajax({
        url: '/add_movie',
        method: 'POST',
        data: payload,
        contentType: 'application/json',
        success: function(response) {
            console.log(response);
            document.location.href="/";
        },
        error: function(xhr, status, error) {
            console.error(status, error);
        }
    });
});
