// calculate dates
function calculateDaysBetweenDates(begin, end) {
    const oneDay = 24 * 60 * 60 * 1000; // milliseconds in a day
    const beginDate = new Date(begin);
    const endDate = new Date(end);

    // Calculate the difference in days
    const diffDays = Math.round(Math.abs((beginDate - endDate) / oneDay));

    return diffDays;
}

//code for fetch youtube video
function fetchYouTubeVideo(videoId) {
    // Code to fetch YouTube video using the provided videoId
    // ...
}
