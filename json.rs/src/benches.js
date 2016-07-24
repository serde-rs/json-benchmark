var chart = require('./chart.js');
var xhr   = require('xhr');

var patternHead = /=+ ([a-z_-]+) =+ parse\|stringify =+(?: parse\|stringify =+)?([^=`]+)/g;
var patternLine = /data\/([^\.]+\.json)\s+([0-9\.]+)ms\s+([0-9\.]+)ms(?:\s+([0-9\.]+)ms\s+([0-9\.]+)ms)?/g;

function cloneTemplate(obj) {
    return Object.assign({}, obj, {data: []});
}

var parseTemplate = {
    label: 'Parse',
    data: [],
    backgroundColor: 'rgba(255, 99, 132, 0.2)',
    borderColor: 'rgba(255, 99, 132, 1)',
    borderWidth: 1
};

var stringifyTemplate = {
    label: 'Stringify',
    data: [],
    backgroundColor: 'rgba(54, 162, 235, 0.2)',
    borderColor: 'rgba(54, 162, 235, 1)',
    borderWidth: 1
};

xhr({
    uri: "https://raw.githubusercontent.com/serde-rs/json-benchmark/master/README.md",
}, function (err, _, body) {
    if (err != null) {
        return console.error("Failed to load the results!", err);
    }

    var files = {};
    var split = body.split(/```/g);
    var dataToParse = (split[1] + "\n" + split[3]).replace(/\s+DOM(?:\s+STRUCT)?/g, '');

    console.log(dataToParse);

    dataToParse.replace(patternHead, function(_, name, results) {
        results.replace(patternLine, function(_, file, sd, pd, ss, ps) {
            files[file] = files[file] || {
                labels: [],
                datasets: [cloneTemplate(parseTemplate), cloneTemplate(stringifyTemplate)]
            };

            files[file].labels.push(name + ' - DOM');
            files[file].datasets[0].data.push(Number(sd));
            files[file].datasets[1].data.push(Number(pd));

            if (ss) {
                files[file].labels.push(name + ' - struct');
                files[file].datasets[0].data.push(Number(ss));
                files[file].datasets[1].data.push(Number(ps));
            }
        });
    });

    console.log(files);

    chart('canada', files['canada.json']);
    chart('citm_catalog', files['citm_catalog.json']);
    chart('twitter', files['twitter.json']);
})

