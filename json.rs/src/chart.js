var Chart = require('chart.js');

module.exports = function chart(id, data) {
    var ctx = document.getElementById(id);

    var myChart = new Chart(ctx, {
        type: 'horizontalBar',
        data: data,
        options: {
            title: {
                display: true,
                text: 'Time in milliseconds (smaller is better)'
            },
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }]
            }
        }
    });
}
